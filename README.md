# cosmwasm-counter-contract

This repository contains a Rust project developed for exploring [CosmWasm](https://cosmwasm.com/).

The smart contract has a state which can be modified by sending `Increment`, `Decrement`, `Reset` and `Set` messages.

The tests are in [`src/contract.rs`](https://github.com/BerzanXYZ/cosmwasm-counter-contract/tree/main/src/contract.rs) file.

This repository also contains JSON schema at [`schema/`](https://github.com/BerzanXYZ/cosmwasm-counter-contract/tree/main/schema) folder.


## Developing
Clone the repository:
```sh
git clone https://github.com/berzanxyz/cosmwasm-counter-contract.git
```

Set current directory:
```sh
cd cosmwasm-counter-contract/
```

Start VS Code:
```sh
code .
```

Reopen the folder in a container:

> VS Code will notify you to reopen the folder in a container. 
>
> Make sure you have Docker installed. 



## Building the smart contract
```sh
cargo wasm-relase # or cargo wasm-debug  
```

## Generating JSON schema 
```sh
cargo schema 
```

## Checking if the smart contract is valid
```sh
cosmwasm-check target/wasm32-unknown-unknown/release/cosmwasm_counter_contract.wasm 
```