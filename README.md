<div align="center">
    <h3><samp>A simple counter example on fuel.</samp></h3>
    <h4><samp>sway contract, indexer, rust sdk contract call, and react app</samp></h4>
</div>

### Pre-requisites

- Get the fuel toolchain [here](https://github.com/FuelLabs/fuelup#quickstart)
- Clone and build the indexer

```shell
git clone https://github.com/FuelLabs/fuel-indexer.git
cargo build --release
```

- Copy the indexer binary into project `bin` folder

```shell
cp target/release/fuel-indexer <project-root>/counter-indexer/bin/
```

### Execution

- Run a local fuel node

```
fuel-core run --db-type in-memory 
```

- Build and test the contract

```
forc build -p counter
cargo test
```

- Run the contract calls

```
cd counter-rust && cargo run
```

- Build and start indexing

```shell
cd counter-indexer

# build target will auto set
cargo build --release

# snip the erands
./wasm_clip.sh

# run the indexer
./bin/fuel-indexer --manifest manifest.yaml

# to run with custom config
./bin/fuel-indexer --manifest manifest.yaml --config config.yaml
```
