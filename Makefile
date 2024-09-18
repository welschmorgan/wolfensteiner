all: rust wasm web

rust:
	cargo b

wasm:
	wasm-pack build --release --target web --out-dir web/pkg

web:
	cd web && npm run build

.PHONY: wasm rust web