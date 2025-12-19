lint: 
	cargo clippy --examples --tests -- -D warnings 
fmt:
	cargo fmt --all 
	cargo clippy --examples --tests --fix --allow-dirty -- -D warnings
build:
	cargo build --all-features
test:
	cargo test --all-features
run:
	cargo run --all-features

codegen:
	curl https://thalex.com/docs/thalex_api.yaml | yq '.' > openapi.json

	python build_scripts/pre-process.py
	rm -rf ./generated

	openapi-generator-cli generate \
	  -i openapi_updated.json \
	  -g rust \
	  -o ./generated \
	--additional-properties=supportAsync=false,useSingleRequestParameter=true
	rm -rf ./src/models/* ./docs/generated/*
	cp ./generated/src/models/* ./src/models/
	cp ./generated/docs/* ./docs/generated/

	# we do the websocket 
	redocly bundle ws_spec.json -o ws_spec_updated.json 

	openapi-generator-cli generate \
	  -i ws_spec_updated.json \
	  -g rust \
	  -o ./generated \
	--additional-properties=supportAsync=false,useSingleRequestParameter=true
	rm -rf ./src/models/* ./docs/generated/* 
	cp ./generated/src/models/* ./src/models/
	cp ./generated/docs/* ./docs/generated/
	rm -rf ./generated

	python build_scripts/post-process.py

all: codegen fmt lint build test
