FROM clux/muslrust:stable AS builder

WORKDIR /app

ENV CC=musl-gcc

COPY . .

RUN cargo clean

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /app

COPY --from=builder \
    /app/target/x86_64-unknown-linux-musl/release/http \
    ./http

EXPOSE 3000

CMD ["./http"]
