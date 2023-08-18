#!/bin/bash

# Random collection of might-be useful links if this ever does not work
# https://www.modio.se/cross-compiling-rust-binaries-to-armv7.html
# https://kerkour.com/rust-cross-compilation
# https://github.com/cross-rs/cross/tree/main

clear
echo "creating local folders"
echo
mkdir ./aarch64build/
mkdir ./aarch64build/target

echo
echo "building image for linux/arm64"
echo
docker build --platform linux/arm64 -t rustarm64 .

echo
echo "running container for linux/arm64"
echo

docker run -it --platform linux/arm64 --name rustcont rustarm64

# from here, container finished its CMD
# copy from container to local
docker cp rustcont:/gamepad-bridge/target/release/gamepad-bridge    ./aarch64build/gamepad-bridge
docker cp rustcont:/gamepad-bridge/target/  ./aarch64build/
docker rm rustcont

echo
echo "project compiled, transfering to raspi"
echo

./buildCopy.sh
