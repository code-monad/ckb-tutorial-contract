# tutorial-contract

## How to generate/init a contract project

### Prerequisite

Follow https://github.com/cryptape/ckb-script-templates

- `git`, `make`, `sed`, `bash`, `sha256sum` (you have these on most unix compatible systems)
- `rust`, follow: https://rustup.rs/ , and do: `rustup target add riscv64imac-unknown-none-elf`
- `llvm` > 16
- `cargo-generate`, `cargo install cargo-generate`

### Generate a project

```bash
cargo generate gh:cryptape/ckb-script-templates workspace --name my-first-contract
cd my-first-contract
```

### Generate a contract inside the project

Generate a contract with a given name

```bash
make generate
🤷   Project Name: my-contract
🔧   Destination: ./my-first-contract/contracts/my-contract ...
🔧   project-name: my-contract ...
🔧   Generating template ...
🔧   Moving generated files into: `./tutorial-contract/contracts/my-contract`...
🔧   Initializing a fresh Git repository
✨   Done! New project created ./tutorial-contract/contracts/my-contract
```

## Implement it!

open `./contracts/my-contract`, fill your logic in `src/main.rs`

## Write your test in `tests/src/tests.rs` !

test cases goes into `tests/tests.rs`, you can also use your own project structure.

## Build it, test it!

1. run `make build MODE=debug` to build your contract
2. run `make test` to test your contract

## Distribute your contract!

0. build your contract a release binary: `make build MODE=release`
1. ensure you have installed ckb-cli: https://github.com/nervosnetwork/ckb-cli
2. generate a deployment info: `ckb-cli --url https://testnet.ckb.dev/rpc deploy init-config --deployment-config deploy.yaml --output-format yaml`
3. generate your deployment tx, and sign it: `mkdir -p ./migration && ckb-cli --url https://testnet.ckb.dev/rpc deploy gen-txs --info-file deploy.toml --migration-dir migration --from-address YOUR_ADDRESS --fee-rate 1000 --deployment-config deploy.yaml --sign-now`
4. send your tx: `deploy apply-txs --info-file deploy.toml --migration-dir ./migration`
