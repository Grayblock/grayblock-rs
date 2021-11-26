bootstrap:
	cargo install --force --locked trunk

dev:
	cd frontend && trunk serve --release

all:
	cd frontend && cargo check && trunk build --release
	cd design && cargo check && trunk build --release
	cd backend && cargo check && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release

ui:
	cd design && trunk serve --release

release:
	cd frontend && trunk build --release
	cd backend && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release

run: release
	target/release/grayblock-backend
