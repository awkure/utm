CARGO  		  ?= cargo 	   #~/.cargo/bin/cargo 	
TARGET 		  := ./target
.DEFAULT_GOAL := all

help:
	@echo 'make		    '
	@echo '     all		'
	@echo '     run		'
	@echo '     tidy	'
	@echo '		test	'
	@echo '     install '
	@echo '     release '

all: release run
	@printf '\n[\x1B[1;32m*\x1B[0m] Done.\n'

install:
	@printf '[\x1B[1;32m+\x1B[0m] Building...'
	@$(CARGO) build -q 

release:
	@printf '[\x1B[1;32m+\x1B[0m] Building...'
	@$(CARGO) build --release -q

run: release
	@printf '\n[\x1B[1;32m*\x1B[0m] Running...'
	@$(CARGO) run --all-features -q

test: 
	@printf '\n[\x1B[1;32m*\x1B[0m] Testing...'
	@$(CARGO) test --all --release --no-fail-fast

tidy:
	@printf '\n[\x1B[1;33m*\x1B[0m] Tidying...'
	@rm -rv $(TARGET)
	@rm -v cargo.lock
	@rm -v ./examples/output/*
	@rm -v ./data/output/*


.PHONY: tidy install release
