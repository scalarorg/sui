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

# HuongND 2023-12-14
reth_test_cluster() {
    BIN_NAME=reth-test-cluster
    WORKING_DIR=/reth
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/scalar/reth-test-cluster/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    docker cp ${BUILDER}:${WORKING_DIR}/test-genesis.json ${SCRIPT_DIR}/test-genesis.json
    docker cp ${SCRIPT_DIR}/test-genesis.json ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
    rm ${SCRIPT_DIR}/test-genesis.json
}

reth_test_client() {
    BIN_NAME=reth-test-client
    WORKING_DIR=/reth
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/scalar/reth-test-cluster/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    docker cp ${BUILDER}:${WORKING_DIR}/test-genesis.json ${SCRIPT_DIR}/test-genesis.json
    docker cp ${SCRIPT_DIR}/test-genesis.json ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
    rm ${SCRIPT_DIR}/test-genesis.json
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

sui_test_validator() {
    BIN_NAME=sui-test-validator
    WORKING_DIR=/scalar
    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/crates/${BIN_NAME}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin
    rm ${SCRIPT_DIR}/${BIN_NAME}
}

sui_cluster_test() {
    BIN_NAME=sui-cluster-test
    WORKING_DIR=/scalar
    BASIC_DIR_NAME=basics
    BASIC_OUTER_DIR=/sui_programmability/examples
    BASIC_PACKAGES_DIR=${BASIC_OUTER_DIR}/${BASIC_DIR_NAME}
    MANAGED_COIN_DIR_NAME=managed_coin
    MANAGED_OUTER_DIR=/crates/sui-core/src/unit_tests/data
    MANAGED_COIN_PACKAGES_DIR=${MANAGED_OUTER_DIR}/${MANAGED_COIN_DIR_NAME}
    PACKAGES_DIR_NAME=packages
    PACKAGES_OUTER_DIR=/crates/sui-framework
    PACKAGES_DIR=${PACKAGES_OUTER_DIR}/${PACKAGES_DIR_NAME}

    docker exec -it ${BUILDER} cargo build --manifest-path ${WORKING_DIR}/crates/${BIN_NAME}/Cargo.toml --profile dev --bin ${BIN_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}/target/${PROFILE}/${BIN_NAME} ${SCRIPT_DIR}/${BIN_NAME}
    docker cp ${SCRIPT_DIR}/${BIN_NAME} ${RUNNER}:/usr/local/bin

    docker exec -it ${RUNNER} mkdir -p ${WORKING_DIR}/crates/sui-test-transaction-builder

    mkdir -p ${SCRIPT_DIR}/${BASIC_DIR_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}${BASIC_PACKAGES_DIR} ${SCRIPT_DIR}
    docker exec -it ${RUNNER} mkdir -p ${WORKING_DIR}${BASIC_PACKAGES_DIR}
    docker cp ${SCRIPT_DIR}/${BASIC_DIR_NAME}/ ${RUNNER}:${WORKING_DIR}${BASIC_OUTER_DIR}

    mkdir -p ${SCRIPT_DIR}/${MANAGED_COIN_DIR_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}${MANAGED_COIN_PACKAGES_DIR} ${SCRIPT_DIR}
    docker exec -it ${RUNNER} mkdir -p ${WORKING_DIR}${MANAGED_COIN_PACKAGES_DIR}
    docker cp ${SCRIPT_DIR}/${MANAGED_COIN_DIR_NAME}/ ${RUNNER}:${WORKING_DIR}${MANAGED_OUTER_DIR}

    mkdir -p ${SCRIPT_DIR}/${PACKAGES_DIR_NAME}
    docker cp ${BUILDER}:${WORKING_DIR}${PACKAGES_DIR} ${SCRIPT_DIR}
    docker exec -it ${RUNNER} mkdir -p ${WORKING_DIR}${PACKAGES_DIR}
    docker cp ${SCRIPT_DIR}/${PACKAGES_DIR_NAME}/ ${RUNNER}:${WORKING_DIR}${PACKAGES_OUTER_DIR}

    rm ${SCRIPT_DIR}/${BIN_NAME}
    rm -rf ${SCRIPT_DIR}/${BASIC_DIR_NAME}
    rm -rf ${SCRIPT_DIR}/${MANAGED_COIN_DIR_NAME}
    rm -rf ${SCRIPT_DIR}/${PACKAGES_DIR_NAME}
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