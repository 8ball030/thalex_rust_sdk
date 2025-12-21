lint: 
	cargo clippy --examples --tests -- -D warnings 
fmt:
	cargo fmt --all 
	cargo fix --allow-dirty
	cargo clippy --examples --tests --fix --allow-dirty -- -D warnings
build:
	cargo build --all-features
test:
	cargo test --all-features -- --nocapture
run:
	cargo run --all-features

codegen:
	curl https://thalex.com/docs/thalex_api.yaml | yq '.' > openapi.json
	curl https://thalex.com/docs/api.yaml | yq '.' > new_schema.json

	python build_scripts/pre-process.py
	python build_scripts/build_ws_schema.py
	rm -rf ./generated

	openapi-generator-cli generate \
	  -i openapi_updated.json \
	  -g rust \
	  -o ./generated \
	--additional-properties=supportAsync=false,useSingleRequestParameter=true
	rm -rf ./src/models/*
	cp ./generated/src/models/* ./src/models/

	# we do the websocket 
	redocly bundle ws_spec.json -o ws_spec_updated.json 

	openapi-generator-cli generate \
	  -i ws_spec_updated.json \
	  -g rust \
	  -o ./generated \
	--additional-properties=supportAsync=false,useSingleRequestParameter=true,avoidBoxedModels=true,generateAliasAsModel=true
	rm -rf ./src/models/*
	cp ./generated/src/models/* ./src/models/
	rm -rf ./generated

	python build_scripts/post-process.py
	python build_scripts/fix_array_types.py ws_spec_updated.json src/models
	python build_scripts/build-ws.py

	rm openapi_updated.json ws_spec_updated.json openapi.json new_schema.json

all: codegen fmt lint build test
