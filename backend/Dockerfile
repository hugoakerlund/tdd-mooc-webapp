FROM rust:1.92 as builder
WORKDIR /app
COPY . .
ENV DATABASE_URL postgres://username:password@localhost/rustapp
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend
COPY .env /app/.env
EXPOSE 3001
CMD ["./backend"]
