# Million Small Deposits Attack

## **Status:** Complete

## Vulnerability
On NEAR, your contract pays for the storage it uses. This means that the more data you store, the more balance you need to cover for storage. If you don't handle these costs correctly (e.g. asking the user to cover their storage usage), then a million little deposits can drain your contract of its funds. Currently it costs ~1 Ⓝ to store 100kb.

## Reference: [Million Small Deposits](https://docs.near.org/develop/contracts/security/storage)