CARGO_VERSION := $(shell cargo version 2>/dev/null)

include .env

.PHONY: build
build:
# ifdef CARGO_VERSION
	cargo build --target wasm32-unknown-unknown --release
# else
# 	@echo "Building substreams target using Docker. To speed up this step, install a Rust development environment."
# 	docker run --rm -ti --init -v ${PWD}:/usr/src --workdir /usr/src/ rust:bullseye cargo build --target wasm32-unknown-unknown --release
# endif

.PHONY: run
run: build
	substreams run substreams.yaml map_events $(if $(START_BLOCK),-s $(START_BLOCK)) $(if $(STOP_BLOCK),-t $(STOP_BLOCK)) $(if $ENDPOINT, -e $(ENDPOINT))

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events $(if $(START_BLOCK),-s $(START_BLOCK)) $(if $(STOP_BLOCK),-t $(STOP_BLOCK)) $(if $ENDPOINT, -e $(ENDPOINT))

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: pack
pack: build
	substreams pack substreams.yaml

.PHONE: sink_postgres
sink_postgres: build
	substreams-sink-sql setup $(DATABASE_URL) substreams.yaml --postgraphile
	substreams-sink-sql run $(DATABASE_URL) substreams.yaml $(if $ENDPOINT, -e $(ENDPOINT)) --on-module-hash-mistmatch=warn

.PHONE: sink
sink: 
	substreams-sink-sql run $(DATABASE_URL) substreams.yaml $(if $ENDPOINT, -e $(ENDPOINT)) --on-module-hash-mistmatch=warn
