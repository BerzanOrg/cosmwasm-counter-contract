FROM alpine

RUN apk add \
    rustup \
    build-base

RUN rustup-init -y

RUN ./root/.cargo/bin/rustup target add wasm32-unknown-unknown

RUN ./root/.cargo/bin/cargo install cosmwasm-check

RUN apk add \
    git 