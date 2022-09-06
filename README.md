# Partisia Core Contracts

## Contracts

| Name                                       | Description                                              |
| ------------------------------------------ | -------------------------------------------------------- |
| [`MPC20`](contracts/mpc20/)                | Implementation of ERC20 Interface                        |
| [`MPC721`](contracts/mpc721/)              | Implementation of ERC721 Interface                       |
| [`MPC1155`](contracts/mpc1155/)            | Implementation of ERC1155 Interface                      |
| [`MPC20-Staking`](contracts/mpc20-staking) | Implementation of ERC20 Interface with staking mechanism |
| [`MPC1-Multisig`](contracts/mpc1-multisig) | On-chain multisig contract                               |

## Test

Run `cargo test` to run all unit tests

## How to build contracts
Currently `partisia-contract` sdk doesn't support Rust Workspaces.
So to build contracts from this repo on your own you will need to:
1. Clone this repo
2. Create a new contract outside this folder
3. Import {contract}-base package from `packages/` folder.
4. Copy all the files from selected contract, for example from `contracts/mpc20`
5. Run `cargo partisia-contract build --release` command.

Or you can download already compiled artifacts from [`here`](https://github.com/partisiablockchainapplications/CoreContracts/releases)