FROM rust:1.60 as build

RUN USER=root
WORKDIR /api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/api*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /api/target/release/api .

CMD ["./api"]