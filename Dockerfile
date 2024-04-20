FROM rust:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    build-essential libssl-dev \
    libclang-dev cmake

COPY . .

RUN cargo build --release