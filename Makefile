all: target/release/solve-davar

target/debug/solve-davar target/release/solve-davar : Cargo.toml src src/bin src/bin/alldown.rs src/davar.rs src/in_out.rs src/main.rs src/opts.rs src/simulate.rs
	cargo build --release && cargo build && cargo doc && RUST_BACKTRACE=1 cargo test

