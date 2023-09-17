#!/usr/bin/env bash


# to see how much space docker takes up:
# docker system df

# clean up EVERYTHING shown in the above command:
# docker system prune -af

# Remove only build cache:
# docker builder prune

clear
echo ">> creating local folders"
echo
mkdir ./aarch64/
mkdir ./aarch64/target
rm ./aarch64/gamepad-bridge

echo
echo ">> building image for linux/arm64"
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

# copy finished build to folder that is tracked by git
cp ./aarch64/target/release/gamepad-bridge ./aarch64/gamepad-bridge

echo
echo ">> project compiled"
echo

if [ $# -eq 0 ]; then
    echo "To automatically copy the built binary over ssh, specify a hostname or ip and password like this: (both optional)"
    echo " $0 hostname password"
    echo " $0 x.x.x.x password"
    exit 0
fi
if [ $# -eq 1 ]; then
    echo "NOTE: You can specify a password to speed things up: (optional)"
    echo " $0 hostname password"
    echo " $0 x.x.x.x password"
    echo

    rpi_host="$1"
    echo ">> copying binary to destination"
    scp "./aarch64/gamepad-bridge" "$rpi_host:~/gamepad-bridge1"
    echo
    echo ">> overwride old binary at destination"
    ssh $rpi_host "sudo mv ~/gamepad-bridge1 ~/gamepad-bridge"
    echo
    echo ">> set uuid bit and owner of new binary to root"
    ssh $rpi_host "sudo chown root:root ~/gamepad-bridge"
    ssh $rpi_host "sudo chmod +s ~/gamepad-bridge"
    echo
    echo ">> success"
else
    rpi_host="$1"
    password="$2"
    sshpass -p "$password" scp "./aarch64/gamepad-bridge" "$rpi_host:~/gamepad-bridge1"
    echo ">> copied binary to destination"
    sshpass -p "$password" ssh $rpi_host "sudo mv ~/gamepad-bridge1 ~/gamepad-bridge"
    echo ">> overwritten old binary at destination"
    sshpass -p "$password" ssh $rpi_host "sudo chown root:root ~/gamepad-bridge"
    sshpass -p "$password" ssh $rpi_host "sudo chmod +s ~/gamepad-bridge"
    echo ">> owner of new binary set to root and uuid bit set"
fi
