# Start from the Rust image
FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && \
    apt-get install -y cmake && \
    apt-get install -y libfontconfig libfontconfig1-dev && \
    apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
