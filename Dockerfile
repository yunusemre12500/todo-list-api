ARG RUST_VERSION="1.78"

FROM rust:${RUST_VERSION}-slim-buster AS build

WORKDIR /usr/src/todo-app-api

COPY . .

RUN cargo build --release

FROM rust:1.78-slim-buster

COPY --from=build /usr/src/todo-app-api/target/release/todo-app-api /usr/local/bin

STOPSIGNAL SIGTERM

CMD [ "todo-app-api" ]