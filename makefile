TOML_FILE := Cargo.toml

# Extract version from TOML
VERSION := $(shell sed -n 's/^version *= *"\(.*\)"/\1/p' $(TOML_FILE))

# Default: bump patch
PATCH_VERSION := $(shell \
	echo $(VERSION) | awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}' \
)

# Allow override
NEW_VERSION ?= $(PATCH_VERSION)

.PHONY: version tag release


version:
	@echo "Current version: $(VERSION)"
	# Update version in Cargo.toml
	@sed -i.bak 's/^version *= *".*"/version = "$(NEW_VERSION)"/' $(TOML_FILE)
	@rm -f $(TOML_FILE).bak
	@echo "Release version: $(NEW_VERSION)"
	cargo check
tag:
	@git tag -a v$(NEW_VERSION) -m "Release v$(NEW_VERSION)"
	@git push origin v$(NEW_VERSION)

package:
	@echo packaging crate
	git add $(TOML_FILE) Cargo.lock
	@git commit -m "Bump version to v$(NEW_VERSION)"
	@git push
	echo added git
	@cargo package

release: version package tag
	@echo "Creating GitHub release v$(NEW_VERSION)"
	@gh release create v$(NEW_VERSION) \
		--title "v$(NEW_VERSION)" \
		--notes "Release v$(NEW_VERSION)"
	@echo "Creating crate release v$(NEW_VERSION)"
	@cargo publish

lint: 
	cargo clippy --benches --examples --tests -- -D warnings 
fmt:
	cargo fix --allow-dirty
	cargo fmt --all 
	cargo clippy --examples --benches --tests --fix --allow-dirty -- -D warnings

build:
	cargo build --all-features
test:
	cargo test -- --nocapture
run:
	cargo run --all-features

codegen:
	curl https://thalex.com/docs/api.yaml | yq '.' > openapi.json

	python build_scripts/pre-process.py
	python build_scripts/build_ws_schema.py
	rm -rf ./generated

	python build_scripts/build-rpc.py
	rm -rf ./src/models/*
	cp ./generated/src/models/* ./src/models/

	# we do the websocket 
	redocly bundle ws_spec.json -o ws_spec_updated.json 

	openapi-generator-cli generate \
	  -i ws_spec_updated.json \
	  -g rust \
	  -o ./generated \
	--type-mappings number=rust_decimal::Decimal \
	--additional-properties=supportAsync=false,useSingleRequestParameter=true,avoidBoxedModels=true,generateAliasAsModel=true
	rm -rf ./src/models/*
	cp ./generated/src/models/* ./src/models/
	rm -rf ./generated

	python build_scripts/post-process.py
	python build_scripts/fix_array_types.py ws_spec_updated.json src/models
	python build_scripts/build-ws.py

	rm ws_spec_updated.json rpc_spec_generated.json

all: codegen fmt lint build test
