### [Re-Entrancy (Not Working - Need a way to reproduce)](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/reentrancy)
- Trying to Reproduce re-entrancy in NEAR smart contract
- Ref: [reentrancy](https://docs.near.org/develop/contracts/security/callbacks)

### [Overflow](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/overflow)
- I we don't use `overflow-checks = true` in `Cargo.toml` it's possible to overflow.

### [signer_account_id - phishing](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/phishing)
- using `signer_account_id` variable for authentication in a NEAR smart contract makes the contract vulnerable to phishing attacks.
