FROM rust:1.81 as builder
WORKDIR /usr/src/lary
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt install -y libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lary /usr/local/bin/lary
CMD ["lary"]
