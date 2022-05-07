all: wasm-build nodejs nodejs-start

nodejs:
	rm -rf node-test/node_modules node-test/yarn.lock
	yarn --cwd node-test

nodejs-start:
	yarn --cwd node-test start

wasm-build:
	wasm-pack build --target nodejs

build:
	cargo build

