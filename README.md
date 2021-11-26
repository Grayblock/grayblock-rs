# Grayblock Power

[![Rust](https://github.com/Grayblock/grayblock-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Grayblock/grayblock-rs/actions/workflows/rust.yml)

Monorepo for Grayblock Power's Rust Projects

## Dependencies

### Frontend

- [trunk](https://docs.rs/crate/trunk) - For WASM builds and livereload, replaces webpack
- [mogwai](https://docs.rs/mogwai) - For vdom, replaces react
- [web3](https://docs.rs/web3) - For web3, replaces ethers

### Backend

- [warp](https://docs.rs/warp) - Server, replaces express

## Usage

To bootstrap dependencies: `make bootstrap`

To run development server: `make dev`

To run production server: `make run`
