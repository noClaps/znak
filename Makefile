build:
	@cargo build -r --workspace
	@cp target/release/znak .

test: build
	@bun test
