## Fuel
A simple counter example with contract and frontend integration along with 
indexer to index the contract

### Execution
- Run a local fuel node
```
fuel-core run --db-type in-memory 
```

- Build and deploy the contract
```
cd counter
forc build

# copy the contract id
forc deploy --url http://127.0.0.1:4000 --unsigned --gas-price 0
```

- Build the indexer
```
cd counter-indexer
# paste the contract id in manifest
cargo build --release
```

- Run the indexer
```
cargo run --bin fuel-indexer -- --manifest <full-project-path>/manifest.yaml
```

- Exeecute the rust code to create transactions
```
cd counter-rust
cargo run -p counter-rust
```

### Links
- [Transactions](https://fuellabs.github.io/block-explorer-v2/address/fuel1yp9mjqxvkj9mk99j6g84yw57t5y2uh320f3nytqpwermnj54s0mq7uyugw)
- Contract Id `0x1b844c9a56aa59e14668ccaaf2113e83227f5d2e149ef5966a317ede691c1a5e`

