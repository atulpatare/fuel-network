## Indexer

### Build
```
cargo build --release
# to remove errant symbols from wasm
./wasm_clip.sh counter_indexer.wasm 
```

### Run
```
cargo run --bin index-runner -- --wasm target/wasm32-unknown-unknown/release/counter_indexer.wasm
```