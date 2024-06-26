#!/bin/sh
DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"

docker build --file ${DIR}/genesis.Dockerfile --output "type=local,dest=./" .
rm -r /tmp/scalar
mkdir -p /tmp/scalar/scalaris/genesis
cp -R genesis/files /tmp/scalar/scalaris/genesis
cp -R genesis/static /tmp/scalar/scalaris/genesis
