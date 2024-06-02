build:
	cargo build --release
release_locally: build
	sudo cp target/release/pyrenv /usr/local/bin/pyrenv
