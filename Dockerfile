FROM ubuntu:latest
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.70.0 -y

WORKDIR /app
COPY . .

EXPOSE 8080
EXPOSE 5432
EXPOSE 6379

ENTRYPOINT ["./scripts/docker_commands.sh"]
