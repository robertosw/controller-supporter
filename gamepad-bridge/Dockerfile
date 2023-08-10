FROM --platform=linux/arm64 rust:latest

# Install all required system tools
  RUN apt update && apt upgrade -y
  RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross
  RUN apt install -y libudev-dev libsystemd-dev libdbus-1-dev
  # ^ support for the hidapi crate


# prepare project folder
# changes in folder names also effect the shellscript! -> edit there as well
  RUN cd /
  RUN mkdir gamepad-bridge 

  COPY src /gamepad-bridge/src
  COPY Cargo.toml /gamepad-bridge/

  WORKDIR /gamepad-bridge/

# build project 
  RUN rustup target add aarch64-unknown-linux-gnu
  CMD ["cargo", "build", "--release"]


# Stolen from cross-rs
  # ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
  #   CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
  #   CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

