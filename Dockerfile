FROM rust:1.68.0-buster as builder
RUN apt-get update && apt-get install -y cmake musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /bd-ical-creator
COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /bd-ical-creator
COPY --from=builder /bd-ical-creator/target/x86_64-unknown-linux-musl/release/bd-ical-creator /bd-ical-creator/bd-ical-creator
ENTRYPOINT ["/bd-ical-creator/bd-ical-creator"]
