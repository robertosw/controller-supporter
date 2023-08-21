FROM --platform=linux/arm64 rust:latest

# Install all required system tools
  RUN apt update 
  RUN apt upgrade -y
  RUN apt install -y g++-aarch64-linux-gnu
  RUN apt install -y libc6-dev-arm64-cross

  # support for the hidapi crate
  RUN apt install -y libudev-dev
  RUN apt install -y libdbus-1-dev
  RUN apt install -y libsystemd-dev


# prepare project folder
# changes in folder names also effect the shellscript! -> edit there as well
  RUN cd /
  RUN mkdir gamepad-bridge 

  # Keep the Cargo.toml copy first to stop caching
  COPY Cargo.toml /gamepad-bridge/
  COPY aarch64build/target /gamepad-bridge/target
  COPY src /gamepad-bridge/src

  WORKDIR /gamepad-bridge/

# build project 
  RUN rustup target add aarch64-unknown-linux-gnu
  CMD ["cargo", "build", "--release"]


# Stolen from cross-rs
  # ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
  #   CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
  #   CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

