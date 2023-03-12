# Pastecord
Rust reimplementation of Pastecord.

# Getting started
Install `sqlx-cli`
```console
cargo install sqlx-cli
```
Bring up the database:
```console
docker compose up -d
```
Create database and run the migrations:
```console
sqlx db create
sqlx migrate run
```
Start the application:
```console
cargo build --release && ./target/release/pastecord
```