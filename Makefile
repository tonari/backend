DOC_FLAGS := --document-private-items

# Run the program (in debug configuration)
.PHONY: run
run:
	cargo run --features testpages

# Build the program (in release configuration)
.PHONY: build
build:
	cargo build --release

# Document the program
.PHONY: doc
doc:
	cargo doc $(DOC_FLAGS)

# Document the program and open the result in a browser
.PHONY: doc_open
doc_open:
	cargo doc $(DOC_FLAGS) --open

# Automatically format the code
.PHONY: fmt
fmt:
	cargo fmt

# Check the code for errors
.PHONY: check
check:
	cargo clippy

# Clean any built files
.PHONY: clean
clean:
	cargo clean
