FROM rust:1.70.0

WORKDIR app
COPY . .
RUN ["chmod", "+x", "./scripts/docker_commands.sh"]
RUN ["chmod", "+x", "./scripts/fix_prisma_str.sh"]
RUN cargo build --release
RUN cargo build -p prisma-cli --release

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./scripts/docker_commands.sh"]
