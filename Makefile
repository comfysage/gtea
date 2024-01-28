PREFIX ?= ${HOME}/.local

all: gtea

clean:
	-cargo clean
	-rm -rf target

install: gtea
	cargo build --release

doc: readme

readme: all
	@help=$$(./target/release/gtea 2>&1) envsubst < README.template.md > README.md

test:
	cargo test

.PHONY: all clean install doc readme test
