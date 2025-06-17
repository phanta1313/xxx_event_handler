FROM rust:1.87 as builder
WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --jobs $(nproc)

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/xxx_event_handler /usr/local/bin/

CMD ["xxx_event_handler"]