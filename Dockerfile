FROM rust:1.67 as build

RUN USER=root
WORKDIR /api

COPY . .
RUN cargo build --release


FROM debian:bullseye-slim
COPY --from=build /api/target/release/shoppa-api .

CMD ["./shoppa-api"]