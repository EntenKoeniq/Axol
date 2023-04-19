FROM rustlang/rust:nightly

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y nasm xorriso build-essential grub-pc-bin grub2-common
RUN rustup component add rust-src

VOLUME /root/env
WORKDIR /root/env