#!/usr/bin/env bash


# to see how much space docker takes up:
# docker system df

# clean up EVERYTHING shown in the above command:
# docker system prune -af

# Remove only build cache:
# docker builder prune

clear
echo "creating local folders"
echo
mkdir ./aarch64/
mkdir ./aarch64/target

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

docker compose up --build aarch64-build

# delete all containers created by docker-compose
docker compose down

# copy finished build to folder that is tracked by git
cp ./aarch64/target/release/gamepad-bridge ./aarch64/gamepad-bridge

echo
echo "project compiled"
echo
