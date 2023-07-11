FROM rust:1.70.0

WORKDIR app
COPY . .
RUN cargo build --release

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./target/release/game-backend"]
