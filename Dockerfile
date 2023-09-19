FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY . .
RUN apt update && apt install -y librust-openssl-dev libssl-dev 
#RUN rustup install x86_64-unknown-linux-gnu
RUN rustup target add x86_64-unknown-linux-gnu
RUN apt -y update
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_gnu=gcc
#ENV PKG_CONFIG_SYSROOT_DIR=/usr/x86_64-linux-gnu/
#ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig/openssl.pc
#ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
#ENV OPENSSL_DIR=/usr/include/openssl

#RUN cargo build --target x86_64-unknown-linux-musl --release
