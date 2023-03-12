FROM lukemathwalker/cargo-chef:latest-rust-1-slim-buster as chef
WORKDIR /app
RUN apt update && apt install lld clang libssl-dev pkg-config -y 

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin pastecord

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pastecord pastecord
COPY static static
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./pastecord"]