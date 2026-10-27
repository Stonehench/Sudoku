from rust:bookworm as development

workdir /app

run cargo install cargo-watch


from rust:bookworm as builder

workdir /app

copy server /app
copy solver /solver
copy migration /migration

run cargo build --release


from rust:bookworm as prod

workdir /app

copy --from=builder /app/target/release/server .

cmd ["/app/server"]