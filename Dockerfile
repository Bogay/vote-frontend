FROM rust:slim AS base

RUN apt update -y && apt install -y pkg-config libssl-dev
RUN cargo install --locked cargo-leptos

FROM base AS builder

WORKDIR /vote-frontend
COPY . .
RUN cargo leptos build --release

# Because some crates depend on openssl, use same base image as workaround
FROM base

COPY --from=builder /vote-frontend/target/site /vote-frontend/target/server/release/vote /srv/vote/
WORKDIR /srv/vote

# CMD ["/srv/vote/vote"]
CMD ["cargo", "leptos", "serve", "--release"]
