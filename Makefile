i:
	cargo build --release --bin installer && sudo ./target/release/installer
r:
	sudo cargo run --bin env-warden