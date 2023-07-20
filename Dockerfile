FROM rust:1.70.0

WORKDIR app
COPY . .
RUN ["chmod", "+x", "./docker_commands.sh"]
RUN cargo build --release
RUN cargo build -p prisma-cli --release

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./docker_commands.sh"]
