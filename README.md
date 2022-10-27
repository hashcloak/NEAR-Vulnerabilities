### [Re-Entrancy (Not Working - Need a way to reproduce)](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/reentrancy)
- Trying to Reproduce re-entrancy in NEAR smart contract
- Ref: [reentrancy](https://docs.near.org/develop/contracts/security/callbacks)

### [Overflow](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/overflow)
- If we don't use `overflow-checks = true` in `Cargo.toml` it's possible to overflow.

### [signer_account_id - phishing](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/phishing)
- using `signer_account_id` variable for authentication in a NEAR smart contract makes the contract vulnerable to phishing attacks.

### [Million Small Deposits](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/Million_Small_Deposits)
- On NEAR, your contract pays for the storage it uses. This means that the more data you store, the more balance you need to cover for storage. If you don't handle these costs correctly (e.g. asking the user to cover their storage usage), then a million little deposits can drain your contract of its funds. Ref: [Million Small Deposits](https://docs.near.org/develop/contracts/security/storage)

### Ensure it is the User (1yⓃ)
- NEAR uses a system of [Access Keys](https://docs.near.org/concepts/basics/accounts/access-keys) to simplify handling accounts. There are basically two type of keys: `Full Access`, that have full control over an account (i.e. can perform all actions), and `Function Call`, that only have permission to call a specified smart contract's method(s) that do not attach Ⓝ as a deposit.
- When a user [signs in on a website](https://docs.near.org/develop/integrate/frontend#user-sign-in) to interact with your contract, what actually happens is that a `Function Call` key is created and stored in the website. Since the website has access to the `Function Call` key, it can use it to call the authorized methods as it pleases. While this is very user friendly for most cases, it is important to be careful in scenarios involving transferring of valuable assets like NFTs or FTs. In such cases, you need to ensure that the person asking for the asset to be transfer is actually the user.
- One direct and inexpensive way to ensure that the user is the one calling is by requiring to attach 1 yⓃ. In this case, the user will be redirected to the wallet and be asked to accept the transaction. This is because, once again, only the `Full Access` key can be used to send NEAR. Since the `Full Access` key is only in the user's wallet, you can trust that a transaction with 1 yⓃ was made by the user.
