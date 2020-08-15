# syntax=docker/dockerfile:experimental
FROM rust:1.45

RUN apt-get update && apt-get install -y libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang
RUN rustup component add rustfmt

WORKDIR /usr/src/solana

RUN git clone https://github.com/solana-labs/solana --depth=1 --branch v1.3.3

RUN cd solana && patch -p1 ../Add_crypto_syscalls.patch

RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
    --mount=type=cache,target=solana/target \
    cd solana && \
    cargo build --release && \
    mkdir -p /opt/solana/deps && \
    cp $(find ./target/release -maxdepth 1 -type f ! -name "*.*") /opt/solana && \
    cp ./target/release/deps/*.so /opt/solana/deps

ENV PATH="/opt/solana:${PATH}"
CMD ["/usr/src/solana/solana/run.sh"]