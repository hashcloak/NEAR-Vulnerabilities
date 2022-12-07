# Getting Started
- Follow this [instructions](https://doc.rust-lang.org/book/ch01-01-installation.html#installation) to install rust. 
- Add the WASM (WebAssembly) target to our toolchain
```shell
rustup target add wasm32-unknown-unknown
```
- Move to the specific directory of specific vulnerability and Compile the contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```
- Follow below to steps to deploy the contract on chain:
    - First Login your Account
    ```bash
    near login
    ```
    - Giving Our Contract a Name
    ```bash
    near create-account CONTRACT_NAME.ACCOUNT_ID --masterAccount ACCOUNT_ID
    ```
    - Deploy the contract
    ```Shell
    near deploy --wasmFile target/wasm32-unknown-unknown/release/<filename>.wasm --accountId CONTRACT_ID --initFunction init_function_name --initArgs '{"key": "value", "key": value}'
    ```
- Interacting with the contract
    - Calling a function
    ```bash
    near call CONTRACT_ID function_name_to_call '{"key": "value", "key": value}' --accountId ACCOUNT_ID
    ```
**Now you can play with the function to reproduce the issues.**


# List of vulnerabilities

### [Re-Entrancy (Not Working - Need a way to reproduce)](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/reentrancy)
**Status:** Incomplete
- Trying to Reproduce re-entrancy in NEAR smart contract
- Ref: [reentrancy](https://docs.near.org/develop/contracts/security/callbacks)

### [Overflow](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/overflow)
**Status:** Complete
- If we don't use `overflow-checks = true` in `Cargo.toml` it's possible to overflow.

### [signer_account_id - phishing](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/phishing)
**Status:** Complete
- using `signer_account_id` variable for authentication in a NEAR smart contract makes the contract vulnerable to phishing attacks.

### [Million Small Deposits](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/Million_Small_Deposits)
**Status:** Complete
- On NEAR, your contract pays for the storage it uses. This means that the more data you store, the more balance you need to cover for storage. If you don't handle these costs correctly (e.g. asking the user to cover their storage usage), then a million little deposits can drain your contract of its funds. Ref: [Million Small Deposits](https://docs.near.org/develop/contracts/security/storage)

### [Error Prone Patterns](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern)
**Status:** Incomplete
- **[Bug 1](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern/Bug_1)** : Should not replace any collections without clearing state, this will reset any metadata, such as the number of elements, leading to bugs. If you replace the collection with something with a different prefix, it will be functional, but you will lose any previous data and the old values will not be removed from storage.
- **[Bug 2](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern/Bug_2)** : Should not use the same prefix as another collection or there will be unexpected side effects.

### [Error not handled](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/UnsafeCall)
**Status:** Complete
- Call external function without check the return value.