build:
	cargo build --release

install:
	mv ./target/release/todo-app-api /usr/local/bin