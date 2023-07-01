# game-website-backend

Backend for a website to play silly games online.

### Running locally
Make sure you have a compatible version of [Rust](https://www.rust-lang.org/tools/install) and
[Postgres](https://www.postgresql.org/download/) installed.

The backend expects a Postgres database named `game-db` to be running on `localhost:5432`. To create one, run
```shell
createdb game-db
```
You'll also need to create a `.env` file at the project root pointing to your local database URL. The file should look
something like:
```
DATABASE_URL="postgres://[username]:[password]@localhost:5432/game-db"
```
To regenerate the Prisma schema with the new config, run
```shell
cargo prisma db push
```
Afterwards, running
```shell
cargo run
```
will run the server locally on `localhost:8080`.
