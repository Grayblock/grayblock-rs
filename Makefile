bootstrap:
	cargo install --force --locked trunk

dev:
	cd frontend && trunk serve

build:
	cd frontend && trunk build --release
	cd backend && cargo build --release

run: build
	target/release/grayblock-backend
