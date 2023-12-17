FROM rust:1.70.0

WORKDIR /app
RUN cargo build -p prisma-cli --release
RUN ./target/release/prisma-cli generate
RUN ./scripts/fix_prisma_str.sh
RUN cargo build --release
COPY . .

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./scripts/docker_commands.sh"]
