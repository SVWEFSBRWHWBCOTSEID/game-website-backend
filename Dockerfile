FROM rust:1.70.0

WORKDIR /app
COPY . .

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./scripts/docker_commands.sh"]
