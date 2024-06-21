#!/bin/sh
DIR="$( cd "$( dirname "$0" )" && pwd )"

up() {
    docker-compose -f ${DIR}/docker-compose.yaml --env-file ${DIR}/.env up -d
}

down() {
    docker-compose -f ${DIR}/docker-compose.yaml down
}

$@