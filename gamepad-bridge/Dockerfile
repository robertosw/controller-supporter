FROM --platform=linux/arm64 rust:1.71.0
# Since the raspi runs on debian, it does not have the most up to date packages (like glibc)
# The debian version of this container has to fit the debian version of the raspi

# Show debian version:
# lsb_release -a

# https://hub.docker.com/layers/library/rust/1.71.1/images/sha256-79ddef683780336ce47c56c86184cf49e4f36c598d8f0bfe9453f52437b1b9a9?context=explore
# rust:1.71.1 is based on debian 11

# https://hub.docker.com/layers/library/rust/1.72.0/images/sha256-1119497a1d5e1eeffc369e93b7d014a2df5b57e2c63ade082bfda32f594e5192?context=explore
# starting with 1.72.0, this container is based on debian 12


# Install all required system tools
  RUN apt update 
  RUN apt upgrade -y
  RUN apt install -y g++-aarch64-linux-gnu
  RUN apt install -y libc6-dev-arm64-cross

  # support for the hidapi crate
  RUN apt install -y libudev-dev
  RUN apt install -y libdbus-1-dev
  RUN apt install -y libsystemd-dev

  RUN ldd --version && sleep 2
  RUN lsb_release -a && sleep 2

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

