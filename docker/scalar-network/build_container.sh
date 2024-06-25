#!/bin/sh
DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
CONTAINER_BUILDER=scalar-builder
DOCKERFILE="$DIR/runner.Dockerfile"
GIT_REVISION="$(git describe --always --abbrev=12 --dirty --exclude '*')"
BUILD_DATE="$(date -u +'%Y-%m-%d')"
PROFILE=release


docker-compose -f ${DIR}/docker-compose-builder.yaml up -d
cd ${REPO_ROOT}
docker cp Cargo.toml ${CONTAINER_BUILDER}:/workspace
docker cp Cargo.lock ${CONTAINER_BUILDER}:/workspace
docker cp consensus ${CONTAINER_BUILDER}:/workspace
docker cp crates ${CONTAINER_BUILDER}:/workspace
docker cp sui-execution ${CONTAINER_BUILDER}:/workspace
docker cp narwhal ${CONTAINER_BUILDER}:/workspace
docker cp external-crates ${CONTAINER_BUILDER}:/workspace
docker exec ${CONTAINER_BUILDER} cargo build --profile release --bin scalar
docker cp ${CONTAINER_BUILDER}:/workspace/target/release/scalar ${REPO_ROOT}/scalar
docker build -f "$DOCKERFILE" "$REPO_ROOT" \
	--build-arg GIT_REVISION="$GIT_REVISION" \
	--build-arg BUILD_DATE="$BUILD_DATE" \
	--build-arg PROFILE="$PROFILE" \
	"$@"
rm ${REPO_ROOT}/scalar
