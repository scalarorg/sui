#!/bin/sh
SCRIPT_DIR="$( cd "$( dirname "$0" )" && pwd )"
RUNNER=scalar-runner

scalar_cluster() {
  docker exec -it ${RUNNER} /entry.sh scalar_cluster
}
scalar_reth() {
  docker exec -it ${RUNNER} /entry.sh scalar_reth
}

reth() {
  docker exec -it ${RUNNER} /entry.sh reth
}

scalar() {
  docker exec -it ${RUNNER} /entry.sh scalar
}

consensus() {
  docker exec -it ${RUNNER} /entry.sh consensus
}

tss() {
  docker exec -it ${RUNNER} /entry.sh tss
}

relayer() {
  docker exec -it ${RUNNER} /entry.sh relayer
}

$@