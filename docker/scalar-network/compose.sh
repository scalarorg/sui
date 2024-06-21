#!/bin/sh
DIR="$( cd "$( dirname "$0" )" && pwd )"
docker-compose -f ${DIR}/docker-compose.yaml --env-file ${DIR}/.env up -d
