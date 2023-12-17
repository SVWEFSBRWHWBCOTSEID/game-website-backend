FROM rust:1.70.0

RUN cargo build -p prisma-cli --release
RUN ./target/release/prisma-cli generate
RUN ./scripts/fix_prisma_str.sh
RUN cargo build --release

WORKDIR /app
COPY . .

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./scripts/docker_commands.sh"]
