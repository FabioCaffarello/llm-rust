.PHONY: build new-project run-composer check-integration format help

CARGO_CMD := cargo
DOCKER_COMPOSE_CMD := docker-compose
NEW_PROJECT_NAME := playground
NEW_PROJECT_PATH := apps/$(NEW_PROJECT_NAME)

guard-%:
	@ if [ "${${*}}" = "" ]; then \
		echo "Environment variable $* not set"; \
		exit 1; \
	fi


build:
	$(CARGO_CMD) build

new-project: guard-project_type
	$(CARGO_CMD) new $(call project_path) --name $(PROJECT_NAME) $(if $(findstring lib,$(project_type)),--lib,)

run-composer:
	$(DOCKER_COMPOSE_CMD) up -d

check-integration: guard-project run-composer
	$(CARGO_CMD) test -p $(project) --test integration_test

format:
	$(CARGO_CMD) fmt --quiet

help:
	@echo "Available commands:"
	@echo "  build            Build the project using Cargo"
	@echo "  new-project      Create a new Cargo project named $(NEW_PROJECT_NAME) under apps/"
	@echo "  run-composer     Start services defined in docker-compose.yml"
	@echo "  check-integration Run integration tests (requires services from run-composer)"
	@echo "  format           Format Rust code according to style guidelines"
