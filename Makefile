# Variables
DOCKER_COMPOSE_FILE = tools/docker-compose.yml
RUST_PROJECT_NAME = test
MONGODB_USERNAME = test
MONGODB_PASSWORD = test
WRITES = 10000
READS = 10000

export MONGODB_URI=mongodb://$(MONGODB_USERNAME):$(MONGODB_PASSWORD)@localhost:27017/$(RUST_PROJECT_NAME)?authSource=admin

# Targets and their explanations
help:
	@echo "Available targets:"
	@echo "  build             : Build your Rust project"
	@echo "  test              : Run your Rust unit tests"
	@echo "  up                : Start the MongoDB container"
	@echo "  down              : Stop and remove the MongoDB container"
	@echo "  test              : Run your integration tests with MongoDB"

build:
	cargo build
	@echo "Build completed."

release:
	cargo build --release
	@echo "Build completed."

run:
	cargo run -- --uri mongodb://$(MONGODB_USERNAME):$(MONGODB_PASSWORD)@localhost:27017/$(RUST_PROJECT_NAME)?authSource=admin -w $(WRITES) -r $(READS)

stats:
	cargo run -- stats --uri mongodb://$(MONGODB_USERNAME):$(MONGODB_PASSWORD)@localhost:27017/$(RUST_PROJECT_NAME)?authSource=admin

test:
	cargo test
	@echo "Unit tests completed."

up:
	docker-compose -f $(DOCKER_COMPOSE_FILE) up -d
	@echo "MongoDB container is up and running."

down:
	docker-compose -f $(DOCKER_COMPOSE_FILE) down
	@echo "MongoDB container is stopped and removed."

.PHONY: help build test up down test
