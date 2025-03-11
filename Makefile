ifeq ($(shell uname -s), Darwin)
	CPU_CORES = $(shell sysctl -n hw.ncpu)
else
	CPU_CORES = $(shell grep -c processor /proc/cpuinfo)
endif

.PHONY:	help
help: ## show help message.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY:	check
check: ## check compile is succeed
	@cargo check -j $(CPU_CORES) --lib --bins --tests

.PHONY:	build
build: ## build application (This command cannot update Cargo.toml)
	@cargo test -j $(CPU_CORES) --no-run --locked

.PHONY:	update_cargo
update_cargo: ## build application
	@cargo test -j $(CPU_CORES) --no-run

.PHONY:	release
release: ## build release binary using musl
	@cargo build --release --target x86_64-unknown-linux-musl

.PHONY:	run
run: ## run: cargo run
	@make build
	@cargo run --quiet -j $(CPU_CORES) -- $(ARGS)

run_release: ## run: cargo build --release and run binary
	@make release
	@./target/x86_64-unknown-linux-musl/release/searchy $(ARGS)

.PHONY:	test
test: ## run: only unit test
	@cargo test --lib

.PHONY: test_debug
test_debug: ## run: only unit test (print debug mode)
	@cargo test --lib -- --nocapture

.PHONY: format
format: ## run: cargo clippy && cargo fmt
	@./scripts/cargo_format.sh

.PHONY:	clean
clean: ## run: cargo clean
	@cargo clean

.PHONY:	meilisearch_up
meilisearch_up: ## run: meilisearch server (default port: 7700. Please set 'MEILISEARCH_PORT' environment variable if you want to change port)
	@./scripts/run_meilisearch.sh --up

.PHONY:	meilisearch_down
meilisearch_down: ## shutdown: meilisearch server
	@./scripts/run_meilisearch.sh --down

.PHONY:	meilisearch_log
meilisearch_log: ## show log: meilisearch server
	@./scripts/run_meilisearch.sh --log

.PHONY:	meilisearch_shell
meilisearch_shell: ## into shell: meilisearch server
	@./scripts/run_meilisearch.sh --shell

.PHONY:	dump_from_postgres
dump_from_postgres: ## run: dump data from postgresql
	@./scripts/dump_data_from_postgres.sh
