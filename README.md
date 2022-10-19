### [Re-Entrancy (Not Working - Need a way to reproduce)](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/reentrancy)
- Trying to Reproduce re-entrancy in NEAR smart contract
- Ref: [reentrancy](https://docs.near.org/develop/contracts/security/callbacks)

### [Overflow](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/overflow)
- I we don't use `overflow-checks = true` in `Cargo.toml` it's possible to overflow.

### [signer_account_id - phishing](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/phishing)
- using `signer_account_id` variable for authentication in a NEAR smart contract makes the contract vulnerable to phishing attacks.

### [Million Small Deposits](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/Million_Small_Deposits)
- On NEAR, your contract pays for the storage it uses. This means that the more data you store, the more balance you need to cover for storage. If you don't handle these costs correctly (e.g. asking the user to cover their storage usage), then a million little deposits can drain your contract of its funds. Ref: [Million Small Deposits](https://docs.near.org/develop/contracts/security/storage)
