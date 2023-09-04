PREFIX = ${HOME}/.local

all: gtea

gtea:
	@printf '%s\n\n' '#!/bin/sh -e' >gtea
	@for i in src/* ; do cat $$i ; echo ; done >>gtea
	@echo 'main "$$@"' >>gtea

clean:
	@-rm gtea

install: gtea
	@echo installing gtea to ${PREFIX}/bin/gtea
	@install -D -m 0755 gtea ${PREFIX}/bin/gtea

.PHONY: all
