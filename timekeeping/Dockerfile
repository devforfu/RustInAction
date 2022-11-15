# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust:1.64 as build

ENV NAME=timekeeping
RUN USER=root cargo new --bin $NAME
WORKDIR /$NAME

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/$NAME*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /timekeeping/target/release/timekeeping .

ENV action get
ENV standard timestamp
ENV datetime unset

CMD ["sh", "-c", "./timekeeping ${action} -s ${standard} \"${datetime}\""]
