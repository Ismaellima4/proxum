FROM rust:1.93-alpine3.20 as build

RUN apk add --no-cache musl-dev protobuf-dev

WORKDIR /app
COPY . /app

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/proxum /
CMD ["./proxum"]
