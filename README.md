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

### [Re-Entrancy](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/reentrancy)
**Status:** Incomplete

### [Overflow](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/overflow)
**Status:** Complete

### [signer_account_id - phishing](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/phishing)
**Status:** Complete

### [Million Small Deposits](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/Million_Small_Deposits)
**Status:** Complete

### [Error Prone Patterns](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern)
**Status:** Incomplete
- **[Bug 1](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern/Bug_1)** 
- **[Bug 2](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern/Bug_2)** 

### [Unchecked Return Value](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/UnsafeCall)
**Status:** Complete