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
	@echo "make book            - build the mdBook and its API reference (requires mdbook)"
	@echo "make hooks           - install the git hooks (core.hooksPath=.githooks)"
	@echo "make bootstrap       - first-time project bootstrap"
	@echo "make update-scaffold - pull the latest bedrock spine (set URL=<bedrock-repo>)"

# Everything that compiles or tests goes through the host guard, because this machine is shared: it waits for room, runs at lower
# priority on a third of the cores, and is stopped before it endangers the host (docs/decisions/decision_host-guard.md).
GUARD := python3 -I -B scripts/host_guard.py run --wait 1800 --
# A guard emptied or cut short exits 0 having run nothing, and a recipe through it would read as passed. One broken any other way exits with its error. So this looks at the file and runs nothing.
ifneq ($(strip $(shell tail -n 1 scripts/host_guard.py)),sys.exit(main(sys.argv[1:])))
$(error scripts/host_guard.py is not whole, so nothing here would be guarded or run: restore it)
endif

# ROADMAP.md §14.3, through the runner in xtask/. `incomplete` (exit 20) is NOT a pass: a
# required tool that is unavailable is reported as unavailable.
focused:
	$(GUARD) cargo xtask verify --tier focused

integration:
	$(GUARD) cargo xtask verify --tier integration

tiers:
	$(GUARD) cargo xtask verify --list

gate:
	$(GUARD) scripts/check_doctrines.sh

check:
	cargo fmt --all -- --check
	$(GUARD) cargo clippy --all-targets --all-features -- -D warnings
	$(GUARD) cargo test --all

fmt:
	cargo fmt --all

clippy:
	$(GUARD) cargo clippy --all-targets --all-features -- -D warnings

test:
	$(GUARD) cargo test --all

book:
	$(GUARD) scripts/build_book.sh

hooks:
	git config core.hooksPath .githooks
	@echo "git hooks activated (core.hooksPath=.githooks)"

bootstrap:
	scripts/bootstrap.sh

update-scaffold:
	scripts/update_scaffold.sh $(URL)
