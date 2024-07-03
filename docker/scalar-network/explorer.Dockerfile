# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM node:20.12.2-bookworm AS builder
ARG PROFILE=release
ARG GIT_REVISION
ARG NETWORK
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y git
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" bash -
RUN cp /root/.local/share/pnpm/pnpm /usr/local/bin
RUN git clone https://github.com/scalarorg/sui-explorer.git -b scalar
WORKDIR "$WORKDIR/sui-explorer"
RUN rm pnpm-lock.yaml
RUN pnpm install
WORKDIR "$WORKDIR/sui-explorer/apps/explorer"
RUN echo "VITE_NETWORK=$NETWORK" > .env.production
RUN pnpm run build

# Production Image
FROM nginx:1.27.0-bookworm AS runtime
RUN apt-get update && apt-get install -y ca-certificates curl
COPY --from=builder /sui-explorer/apps/explorer/build /usr/share/nginx/html
COPY explorer.conf /etc/nginx/conf.d/explorer.conf

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
