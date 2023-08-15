# game-website-backend

Backend for a website to play silly games online.

### Running locally
Make sure you have a compatible version of [Rust](https://www.rust-lang.org/tools/install) and
[Postgres](https://www.postgresql.org/download/) installed.

The backend expects a Postgres database named `game-db` to be running on `localhost:5432`. To create one, run
```shell
createdb game-db
```

You'll also need a Redis service running on `redis://127.0.0.1:6379`. Install Redis on Ubuntu via `apt-get`:
```shell
sudo apt-add-repository ppa:redislabs/redis
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install redis-server
```
Run
```shell
sudo service redis-server restart
```
to start the service on `127.0.0.1:6379`.[^1]

[^1]: If you're on Windows, you'll have to [install and run Redis via WSL](https://developer.redis.com/create/windows/).

You'll need to create a `.env` file at the project root pointing to your host, port, local database URL, and redis URL.
The file should look something like:
```
HOST="127.0.0.1"
PORT=8080
DATABASE_URL="postgres://[username]:[password]@localhost:5432/game-db"
REDIS_URL="redis://127.0.0.1:6379"
DOMAIN="127.0.0.1"
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
