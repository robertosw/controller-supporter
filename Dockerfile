# Using the same version as the Dockerfile for building to aarch64
FROM rust:1.71.0

# Install all required system tools
RUN apt update 
RUN apt upgrade -y

# support for the hidapi crate
RUN apt install -y libudev-dev
RUN apt install -y libdbus-1-dev
RUN apt install -y libsystemd-dev

# Rust formatter
RUN rustup component add rustfmt
