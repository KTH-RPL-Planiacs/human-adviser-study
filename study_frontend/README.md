# study_frontend

A WASM app that runs in the browser. This is the interactive study itself. To run inside the browser, try:

```sh
cargo run --target wasm32-unknown-unknown
```

To build all files for the web server:

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/release/study_frontend.wasm --target web --out-dir ../docs --no-typescript
```
