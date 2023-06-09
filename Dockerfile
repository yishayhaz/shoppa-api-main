FROM rust:1.67 as build

RUN USER=root
WORKDIR /api

COPY . .
RUN cargo build --release


FROM debian:bullseye-slim
COPY --from=build /api/target/release/shoppa-api .
COPY --from=build /api/scripts /scripts
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/scripts/mongo_ip.sh"]
CMD ["./shoppa-api"]