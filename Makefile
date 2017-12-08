all: build

build-release:
	cargo build --release

build:
	cargo build

run-release:
	cargo run --release

run:
	cargo run

test:
	cargo test

clean:
	rm -rf target/
