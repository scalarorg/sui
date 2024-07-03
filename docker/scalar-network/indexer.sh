#!/bin/bash
/usr/local/bin/sui-indexer \
    --db-user-name postgres \
    --db-password postgres \
    --db-host indexer_db \
    --db-port 5432 \
    --db-name scalar_indexer \
    --rpc-client-url http://fullnode:9000/ \
    --reset-db \
    --rpc-server-worker
