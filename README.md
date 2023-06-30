# game-website-backend

Backend for a website to play silly games online.

### Running locally
Make sure you have a compatible version of [Rust](https://www.rust-lang.org/tools/install) and
[Postgres](https://www.postgresql.org/download/) installed.

The backend expects a Postgres database named `game-db` to be running on `localhost:5432`. To create one, run
```shell
createdb game-db
```

Running
```shell
cargo run
```
will run the server locally on `localhost:8080`.
