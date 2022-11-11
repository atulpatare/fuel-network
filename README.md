## Fuel
A simple counter example with contract and frontend integration along with 
indexer to index the contract

### Contract
1. Build
```
forc build
```

2. Run tests
```
cargo test
```

3. Deploy on testnet
```
forc deploy --url https://node-beta-1.fuel.network/graphql --gas-price 1
# enter the wallet address
# copy the tx id or message and sign using

forc wallet sign <TX_MSG> <IDX>
# copy the sign message paste it in the deploy terminal
# get the final transaction hash, look it up on explorer
```

4. To add tests
```
cargo generate --init fuellabs/sway templates/sway-test-rs --name counter-contract
```

### Links
- [Transactions](https://fuellabs.github.io/block-explorer-v2/address/fuel1yp9mjqxvkj9mk99j6g84yw57t5y2uh320f3nytqpwermnj54s0mq7uyugw)
- Contract Id `a2f2445abbc6972b4aaa597c756d991da45b9695358f1fc1464fa3776f3799d7`
