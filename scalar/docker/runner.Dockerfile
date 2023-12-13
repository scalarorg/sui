# Production Image
FROM debian:bookworm-slim AS runtime
# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates
# For debuging
RUN apt-get update && apt-get install -y nano curl
ENV LD_PRELOAD /usr/lib/x86_64-linux-gnu/libjemalloc.so
# ARG PROFILE=release
# ARG BUILD_DATE
# ARG GIT_REVISION
# LABEL build-date=$BUILD_DATE
# LABEL git-revision=$GIT_REVISION
WORKDIR /scalar

ENTRYPOINT [ "sleep", "infinity"]