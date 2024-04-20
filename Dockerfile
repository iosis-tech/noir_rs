FROM rust:1.77.2 as base

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    build-essential libssl-dev \
    libclang-dev cmake pkg-config

COPY . .