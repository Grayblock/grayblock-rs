bootstrap:
	rustup update
	rustup component add clippy
	cargo install --force --locked trunk

dev:
	cd frontend && trunk serve --release

all:
	cd frontend && cargo clippy && trunk build --release
	cd design && cargo clippy && trunk build --release
	cd backend && cargo clippy && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
	cargo doc

ui:
	cd design && trunk serve

release:
	cd frontend && trunk build --release
	cd backend && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release

run: release
	target/release/grayblock-backend

clean:
	rm -rf target frontend/dist

ci:
	cargo clippy -- -D warnings
	cargo test
