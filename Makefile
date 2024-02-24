rust-version:
	rustc --version 		# rustc compiler
	cargo --version 		# cargo package manager
	rustfmt --version 		# rust formatter
	rustup --version 		# rust toolchain manager
	clippy-driver --version	# rust linter

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

run:
	cargo run -r

release:
	cargo build -r
	strip target/release/prepare-commit-msg

install: release
	$(eval GIT_HOOKS_PATH := $(shell git config --global core.hooksPath))
	@if [ -z "$(GIT_HOOKS_PATH)" ]; then \
		GIT_HOOKS_PATH="~/.git_hooks"; \
		mkdir -p $(GIT_HOOKS_PATH); \
		git config --global core.hooksPath $(GIT_HOOKS_PATH); \
	fi
	cp -u target/release/prepare-commit-msg $(GIT_HOOKS_PATH)/prepare-commit-msg
	$(MAKE) clean

clean:
	cargo clean
	rm -rf target

uninstall:
	$(eval GIT_HOOKS_PATH := $(shell git config --global core.hooksPath))
	@if [ -z "$(GIT_HOOKS_PATH)" ]; then \
		GIT_HOOKS_PATH="~/.git_hooks"; \
	fi
	rm -f $(GIT_HOOKS_PATH)/prepare-commit-msg