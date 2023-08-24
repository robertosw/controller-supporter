#!/bin/bash


# to see how much space docker takes up:
# docker system df

# clean up EVERYTHING shown in the above command:
# docker system prune -af

# Remove only build cache:
# docker builder prune

clear
echo "creating local folders"
echo
mkdir ./aarch64build/
mkdir ./aarch64build/target

echo
echo "building image for linux/arm64"
echo

# Increment the version in Cargo.toml to ensure that docker doesnt cache the source code

    # Read the current version from Cargo.toml
    current_version=$(grep -oP --max-count=1 'version\s*=\s*"\K[^"]*' Cargo.toml)

    echo current version: $current_version

    # Split the version into its components
    major=$(echo "$current_version" | cut -d'.' -f1)
    minor=$(echo "$current_version" | cut -d'.' -f2)
    patch=$(echo "$current_version" | cut -d'.' -f3 | cut -d'-' -f1)
    build=$(echo "$current_version" | cut -d'-' -f2)

    build=$((build + 1))
    new_version="$major.$minor.$patch-$build"

    echo new version: $new_version

    # replace old with the new version
    sed -i "s/version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml

docker build --platform linux/arm64 -t rustarm64 .

echo
echo "running container for linux/arm64"
echo

docker run -it --platform linux/arm64 --name rustcont rustarm64

# from here, container finished its CMD
# copy from container to local
# docker cp rustcont:/gamepad-bridge/target/debug/gamepad-bridge      ./aarch64build/gamepad-bridge
docker cp rustcont:/gamepad-bridge/target/release/gamepad-bridge    ./aarch64build/gamepad-bridge
docker cp rustcont:/gamepad-bridge/target/  ./aarch64build/
docker container remove rustcont --volumes
docker image prune --force  # delete older versions of this image to not cluster disk

echo
echo "project compiled"
echo

./buildCopy.sh
