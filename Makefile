bootstrap:
	cargo install --force --locked trunk

dev:
	cd frontend && trunk serve

dev-all:
	cd frontend && cargo check && trunk build
	cd design && cargo check && trunk build
	cd backend && cargo check && cargo build

ui:
	cd design && trunk serve

release-build:
	cd frontend && trunk build --release
	cd backend && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release

run: release-build
	target/release/grayblock-backend
