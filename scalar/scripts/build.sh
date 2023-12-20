#!/bin/sh
SCRIPT_DIR="$( cd "$( dirname "$0" )" && pwd )"
PROFILE=debug
RUNNER=scalar-runner
BUILDER=scalar-builder

BIN_DIR=${SCRIPT_DIR}/validator/${PROFILE}
SCALAR_DIR=${SCRIPT_DIR}/../../scalar
TOFND_DIR=${SCRIPT_DIR}/../../../tofnd

# Working from 2023-12-13
scalar_cluster() {
    BIN_NAME=scalar-cluster
    WORKING_DIR=/scalar
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}

# Working from 2023-12-13
scalar_reth() {
    BIN_NAME=scalar-reth
    WORKING_DIR=/reth
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/scalar/reth-node/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}

reth() {
    BIN_NAME=reth
    WORKING_DIR=/reth
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}

scalar() {
    BIN_NAME=scalar-node
    WORKING_DIR=/scalar
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}

consensus() {
    BIN_NAME=consensus-node
    WORKING_DIR=/scalar
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}

sui() {
    BIN_NAME=sui
    RKING_DIR=/scalar
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}



relayer() {
    BUILDER=scalar-relayer
    docker exec -it ${BUILDER} cargo build --manifest-path /scalar-relayer/Cargo.toml --profile dev
    docker cp ${BUILDER}:/scalar-relayer/target/${PROFILE}/scalar-relayer ./scalar-relayer
    docker cp ./scalar-relayer ${RUNNER}:/usr/local/bin/scalar-relayer
    rm ./scalar-relayer
}

tss() {
    docker exec ${BUILDER} rustup component add rustfmt --toolchain 1.70-x86_64-unknown-linux-gnu
    docker exec ${BUILDER} cargo build --manifest-path /tofnd/Cargo.toml --profile dev
    docker cp ${BUILDER}:/tofnd/target/${PROFILE}/tofnd ./tofnd
    docker cp ./tofnd ${RUNNER}:/usr/local/bin/scalar-tofnd
    rm ./tofnd
}

$@