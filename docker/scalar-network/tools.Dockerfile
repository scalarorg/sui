# Production Image
FROM debian:bookworm-slim AS runtime
WORKDIR "$WORKDIR/sui"

# sui-tool needs libpq at runtime
RUN apt-get update && apt-get install -y libpq5 libpq-dev

COPY sui-node /usr/local/bin
COPY sui-bridge /usr/local/bin
COPY sui-bridge-cli /usr/local/bin
COPY sui-proxy /usr/local/bin
COPY sui /usr/local/bin
COPY sui-faucet /usr/local/bin
COPY sui-cluster-test /usr/local/bin
COPY sui-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
