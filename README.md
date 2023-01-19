# Substrate Node Migration

## Overall

The project is based on substrate node template, such as this consists of a number of components that are spread across several directories.

An account *migration_vault_account* was added and pre-funded with total supply coins as per original network. Which is passed to the *migration* pallet configuration.

Additionally, *migration_owner* address is passed to the pallet config as the owner of the migration the only address authorised to perform the action.

The migration pallet has exposed *migrate* method to transfer the received amount from pre-configured migration vault and transfer to the received account.

## Security

The migration pallet can only be triggered by the designated address, who has migration owner role.

## Integrity

As executing transactions uses up tokens and decreases the overall token supply, additional token was created for the gas utility - GAS. This token uses *balances* pallet, while the migrated token uses *assets* pallet. This results exact migration of all the tokens, without loosing anything to the migration process.

## Limitations

While it is possible to use the same private key for the source and destination networks, this is generally considered not recommended practice and anti-pattern. As such, without knowing destination accounts ahead of time, the accounts need to be passed during the migration.

## Getting Started

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Test

The `cargo test` command will perform testing of the pallets.

### Benchmarking

#### Build

The `cargo build --release --features=runtime-benchmarks` command will perform build of the code for the benchmarking.

#### Benchmarking

The `./target/release/node-template benchmark pallet --dev --steps=50 --repeat=20 --pallet=pallet_migration --extrinsic=migrate --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./pallets/migration/src/weights.rs --template=./.maintain/frame-weight-template.hbs` command will perform the benchmarking of the migration pallet.

#### Test

The `cargo test --package pallet-migration --features runtime-benchmarks` command will perform benchmarking testsing of the migration pallet.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
