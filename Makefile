# Makefile — standard commands. `make gate` = the doctrine enforcer; `make check` = Rust.
SHELL := /usr/bin/env bash

.PHONY: help gate check fmt clippy test book hooks bootstrap update-scaffold \
        focused integration tiers

help:
	@echo "make focused         - ROADMAP.md §14.3 focused tier: every edit loop, every ordinary commit"
	@echo "make integration     - §14.3 integration tier: before a push, before closing a milestone"
	@echo "make tiers           - list every tier and what each step proves"
	@echo "make gate            - run the doctrine enforcer (scripts/check_doctrines.sh)"
	@echo "make check           - cargo fmt --check + clippy (deny warnings) + test"
	@echo "make fmt             - cargo fmt --all"
	@echo "make clippy          - cargo clippy --all-targets -- -D warnings"
	@echo "make test            - cargo test --all"
	@echo "make book            - build the mdBook (requires mdbook)"
	@echo "make hooks           - install the git hooks (core.hooksPath=.githooks)"
	@echo "make bootstrap       - first-time project bootstrap"
	@echo "make update-scaffold - pull the latest bedrock spine (set URL=<bedrock-repo>)"

# ROADMAP.md §14.3, through the runner in xtask/. `incomplete` (exit 20) is NOT a pass: a
# required tool that is unavailable is reported as unavailable.
focused:
	cargo xtask verify --tier focused

integration:
	cargo xtask verify --tier integration

tiers:
	cargo xtask verify --list

gate:
	scripts/check_doctrines.sh

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all

book:
	scripts/build_book.sh

hooks:
	git config core.hooksPath .githooks
	@echo "git hooks activated (core.hooksPath=.githooks)"

bootstrap:
	scripts/bootstrap.sh

update-scaffold:
	scripts/update_scaffold.sh $(URL)
