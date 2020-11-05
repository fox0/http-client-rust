MAIN := $(shell basename $(PWD))
SOURCES := $(shell find src/ -name '*.rs')

all: target/release/$(MAIN) target/x86_64-pc-windows-gnu/release/$(MAIN).exe

target/release/%: $(SOURCES)
	cargo build --release

target/x86_64-pc-windows-gnu/release/%: $(SOURCES)
	cargo build --target x86_64-pc-windows-gnu --release

clean:
	cargo clean
