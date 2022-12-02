#!/bin/bash

wasm-snip target/wasm32-unknown-unknown/release/counter_indexer.wasm \
  -o target/wasm32-unknown-unknown/release/counter_indexer.wasm \
  -p __wbindgen
