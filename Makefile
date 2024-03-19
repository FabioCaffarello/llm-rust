.PHONY: build new-project start check-integration format open-doc help

CARGO_CMD := cargo
RUSTUP_CMD := rustup
DOCKER_COMPOSE_CMD := docker-compose
NEW_PROJECT_NAME := playground
NEW_PROJECT_PATH := apps/$(NEW_PROJECT_NAME)

guard-%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1; \
	fi

install:
	@echo "Updating Rust toolchain to the latest stable version"
	$(RUSTUP_CMD) update stable

build:
	$(CARGO_CMD) build

new-project: guard-project_type
	$(CARGO_CMD) new $(call project_path) --name $(PROJECT_NAME) $(if $(findstring lib,$(project_type)),--lib,)

start:
	$(DOCKER_COMPOSE_CMD) up -d

check-integration-bk: guard-project start
	$(CARGO_CMD) test -p $(project) --test integration_test

check-integration: start
	. hack/check-integration.sh $(project)

open-doc:
	$(CARGO_CMD) doc --no-deps --open

format:
	$(CARGO_CMD) fmt --quiet

lint:
	. hack/lint.sh

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version            #rust compiler
	cargo --version             #rust package manager
	rustfmt --version           #rust code formatter
	rustup --version            #rust toolchain manager
	clippy-driver --version     #rust linter

help:
	@echo "Available commands:"
	@echo "  build              Build the project using Cargo"
	@echo "  install            Install required tools and dependencies"
	@echo "  new-project        Create a new Cargo project named $(NEW_PROJECT_NAME) under apps/"
	@echo "  start              Start services defined in docker-compose.yml"
	@echo "  check-integration  Run integration tests (requires services from start)"
	@echo "  format             Format Rust code according to style guidelines"
	@echo "  lint               Run linters"
	@echo "  open-doc           Open the documentation in a web browser"
	@echo "  rust-version       Display the versions of Rust command-line utilities"
