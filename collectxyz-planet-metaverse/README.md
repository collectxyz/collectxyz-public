# xyz Planets Metaverse Contracts

This repository contains smart contracts that implement the xyz Planets Metaverse on top of the [xyz NFT contract](https://github.com/collectxyz/collectxyz-nft-contract).

## Development

### Environment Setup

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker

1. Install `rustup` via https://rustup.rs/

2. Run the following:

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Make sure [Docker](https://www.docker.com/) is installed

### Testing

Run all tests for the workspace:

```sh
cargo test
```

### Compiling

To compile the a contract, first `cd` into that contract's subdirectory in `contracts/`, then run:

```sh
RUSTFLAGS='-C link-arg=-s' cargo wasm
shasum -a 256  ../../target/wasm32-unknown-unknown/release/$CONTRACT_NAME.wasm
```

#### Production

For production builds, run the following:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.11.5
```

This uses [rust-optimizer](https://github.com/cosmwasm/rust-optimizer) to perform several optimizations which can significantly reduce the final size of the contract binaries, which will be available inside the `artifacts/` directory.
