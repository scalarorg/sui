#!/bin/sh
DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
CONTAINER_BUILDER=scalar-builder
DOCKERFILE="$DIR/runner.Dockerfile"
GIT_REVISION="$(git describe --always --abbrev=12 --dirty --exclude '*')"
BUILD_DATE="$(date -u +'%Y-%m-%d')"
PROFILE=release

scalar() {
	docker-compose -f ${DIR}/docker-compose-builder.yaml up -d
	cd ${REPO_ROOT}
	docker cp Cargo.toml ${CONTAINER_BUILDER}:/workspace
	docker cp Cargo.lock ${CONTAINER_BUILDER}:/workspace
	docker cp consensus ${CONTAINER_BUILDER}:/workspace
	docker cp crates ${CONTAINER_BUILDER}:/workspace
	docker cp sui-execution ${CONTAINER_BUILDER}:/workspace
	docker cp narwhal ${CONTAINER_BUILDER}:/workspace
	docker cp external-crates ${CONTAINER_BUILDER}:/workspace
	declare -a bins=("scalar")
	CMD_BINS="";
	for bin in "${bins[@]}"
	do
		CMD_BINS="${CMD_BINS} --bin ${bin}"
	done
	docker exec ${CONTAINER_BUILDER} cargo build --profile ${PROFILE} ${CMD_BINS} 
	for bin in "${bins[@]}"
	do
		docker cp ${CONTAINER_BUILDER}:/workspace/target/release/${bin} ${REPO_ROOT}/${bin}
	done
	docker build -f "$DOCKERFILE" "$REPO_ROOT" \
		--build-arg GIT_REVISION="$GIT_REVISION" \
		--build-arg BUILD_DATE="$BUILD_DATE" \
		--build-arg PROFILE="$PROFILE" \
		"$@"
		
	for bin in "${bins[@]}"
	do
		rm ${REPO_ROOT}/${bin}
	done
}
indexer() {
	docker-compose -f ${DIR}/docker-compose-builder.yaml up -d
	cd ${REPO_ROOT}
	BIN=sui-indexer
	docker cp Cargo.toml ${CONTAINER_BUILDER}:/workspace
	docker cp Cargo.lock ${CONTAINER_BUILDER}:/workspace
	docker cp consensus ${CONTAINER_BUILDER}:/workspace
	docker cp crates ${CONTAINER_BUILDER}:/workspace
	docker cp sui-execution ${CONTAINER_BUILDER}:/workspace
	docker cp narwhal ${CONTAINER_BUILDER}:/workspace
	docker cp external-crates ${CONTAINER_BUILDER}:/workspace
	docker exec ${CONTAINER_BUILDER} cargo build --profile ${PROFILE} --bin ${BIN} --features postgres-feature
	docker cp ${CONTAINER_BUILDER}:/workspace/target/release/${BIN} ${REPO_ROOT}/${BIN}
	docker build -f "$DIR/indexer.Dockerfile" "$REPO_ROOT" \
		--build-arg GIT_REVISION="$GIT_REVISION" \
		--build-arg BUILD_DATE="$BUILD_DATE" \
		--build-arg PROFILE="$PROFILE" \
		"$@"
	rm ${REPO_ROOT}/${BIN}
}

explorer() {
	echo "Building scalar explorer"
	NETWORK=LOCAL
	docker build -f "$DIR/explorer.Dockerfile" "$REPO_ROOT/docker/scalar-network" \
	--build-arg NETWORK="$NETWORK" \
	 "$@"
}

genesis() {
	OUTDIR=${1:-/tmp/scalar}
	docker build --file ${DIR}/genesis.Dockerfile --output "type=local,dest=./" .
	rm -r $OUTDIR
	mkdir -p ${OUTDIR}/genesis
	cp -R genesis/files ${OUTDIR}/genesis
	cp -R genesis/static ${OUTDIR}/genesis
}

tools() {
	docker-compose -f ${DIR}/docker-compose-builder.yaml up -d
	cd ${REPO_ROOT}
	docker cp Cargo.toml ${CONTAINER_BUILDER}:/workspace
	docker cp Cargo.lock ${CONTAINER_BUILDER}:/workspace
	docker cp consensus ${CONTAINER_BUILDER}:/workspace
	docker cp crates ${CONTAINER_BUILDER}:/workspace
	docker cp sui-execution ${CONTAINER_BUILDER}:/workspace
	docker cp narwhal ${CONTAINER_BUILDER}:/workspace
	docker cp external-crates ${CONTAINER_BUILDER}:/workspace
	declare -a bins=("sui-node" "sui-bridge" "sui-bridge-cli" "sui-proxy" "sui" "sui-faucet" "sui-cluster-test" "sui-tool")
	# declare -a bins=("sui-node" "sui-bridge" "sui-bridge-cli" "sui-proxy" "sui" "sui-faucet" "sui-analytics-indexer" "sui-cluster-test" "sui-tool")
	CMD_BINS="";
	for bin in "${bins[@]}"
	do
		CMD_BINS="${CMD_BINS} --bin ${bin}"
	done
	docker exec ${CONTAINER_BUILDER} cargo build --profile ${PROFILE} ${CMD_BINS}
	for bin in "${bins[@]}"
	do
		docker cp ${CONTAINER_BUILDER}:/workspace/target/release/${bin} ${REPO_ROOT}/${bin}
	done

	docker build -f "$DIR/tools.Dockerfile" "$REPO_ROOT" \
		--build-arg GIT_REVISION="$GIT_REVISION" \
		--build-arg BUILD_DATE="$BUILD_DATE" \
		--build-arg PROFILE="$PROFILE" \
		"$@"
	for bin in "${bins[@]}"
	do
		rm ${REPO_ROOT}/${bin}
	done
}

$@