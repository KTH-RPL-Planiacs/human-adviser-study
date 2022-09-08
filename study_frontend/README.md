# study_frontend

a WASM app that runs in the browser. this is the interactive study itself. to run inside the browser, try:

```sh
cargo run --target wasm32-unknown-unknown
```

To build all files for the web server:

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
```
