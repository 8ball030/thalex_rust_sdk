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

	openapi-generator-cli generate \
	  -i openapi_updated.json \
	  -g rust \
	  -o ./generated \
	--additional-properties=supportAsync=false,useSingleRequestParameter=true


	rm -rf ./src/models/* ./docs/generated/*
	cp ./generated/src/models/* ./src/models/
	cp ./generated/docs/* ./docs/generated/
# 	cp -r ./generated/src/apis ./src/

	# rebuild mod.rs
	@echo "#![allow(clippy::all)]" > ./src/models/mod.rs
	@echo "#![allow(unused_imports)]" >> ./src/models/mod.rs
	@echo "#![allow(dead_code)]" >> ./src/models/mod.rs
	@echo "#![allow(non_camel_case_types)]" >> ./src/models/mod.rs
	@echo "#![allow(clippy::upper_case_acronyms)]" >> ./src/models/mod.rs
	# rebuild api mod.rs

# 	@echo "#![allow(clippy::all)]" > ./src/apis/mod.rs
#	cat ./generated/src/apis/mod.rs >> ./src/apis/mod.rs

	# cleanup
	rm -rf ./generated

	@for f in ./src/models/*.rs; do \
		base=$$(basename $$f); \
		if [ "$$base" = "mod.rs" ]; then continue; fi; \
		name=$${base%.rs}; \
		camel=$$(echo $$name | sed -E 's/(^|_)([a-z])/\U\2/g'); \
		camel=$$(echo $$camel | sed -E 's/_//g'); \
		echo "pub mod $$name;" >> ./src/models/mod.rs; \
		echo "pub use $$name::$$camel;" >> ./src/models/mod.rs; \
	done
	python build_scripts/post-process.py

all: codegen fmt lint build test
