.PHONY: check test build dashboard-build ci dev-start

check:
	cargo check --workspace --manifest-path ./Cargo.toml

test:
	cargo test --workspace --manifest-path ./Cargo.toml

build:
	cargo build --workspace --manifest-path ./Cargo.toml

dashboard-build:
	cd dashboard-ui && npx vite build

ci: check test dashboard-build

dev-start:
	powershell -ExecutionPolicy Bypass -File .\deploy\scripts\start-autoloop.ps1

