FROM rust:1.70.0

WORKDIR /app
COPY ./target/release/game-backend .
COPY ./target/release/prisma-cli .
COPY ./scripts/docker_commands.sh .

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./docker_commands.sh"]
