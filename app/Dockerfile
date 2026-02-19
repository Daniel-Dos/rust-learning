FROM rust:1.93-slim as build-env
WORKDIR /app
COPY . /app
COPY user-rust.db /app/user-rust.db
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/app /
COPY --from=build-env /app/user-rust.db /user-rust.db
CMD ["./app"]