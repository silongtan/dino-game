lint:
	cargo clippy --quiet

build-release:
	cargo build --release 

run:
	cargo run 

all: run build-release