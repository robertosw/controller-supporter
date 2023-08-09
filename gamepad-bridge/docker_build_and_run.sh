#!/bin/bash
echo "building for linux/arm64"
echo
docker build --platform linux/arm64 -t rustarm64 .

echo
echo "running container for linux/arm64"
echo
docker run --platform linux/arm64 -it --rm  rustarm64
