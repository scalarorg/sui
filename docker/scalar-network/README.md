# Sui Network Docker Compose

This was tested using MacOS 14.3.1, Docker Compose: v2.13.0.

This compose brings up 3 validators, 1 fullnode, and 1 stress (load gen) client

Steps for running:

0. Prepare Scalaris image
Follow document in scalaris repo


1. Build scalar image

```
cd docker/scalar-network
./build.sh -t scalar/execution:latest

```

Build scalar within running container

```
cd docker/scalar-network
./build.sh scalar -t scalar/execution:latest

```

2. Build genesis (Required for first startup)

```
cd docker/scalar-network
./build.sh genesis
```

3. Build sui tool

 ```
cd docker/scalar-network
./build.sh tools -t scalar/sui-tools:latest

``` 
4. run compose

```
(optional) `rm -r /tmp/scalar`
docker compose up
```

# Demo step

1. Init client
```
sui client
cp $DIR/sui_config/genesis.blob /tmp/scalar/scalaris/genesis/files
```


**additional info**
The version of `sui` which is used to generate the genesis outputs much be on the same protocol version as the fullnode/validators (eg: `mysten/sui-node:mainnet-v1.19.1`)
Here's an example of how to build a `sui` binary that creates a genesis which is compatible with the release: `v1.19.1`
```
git checkout releases/sui-v1.19.0-release
cargo build --bin sui
```
you can also use `sui-network/Dockerfile` for building genesis
