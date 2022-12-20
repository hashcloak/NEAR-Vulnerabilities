# signer_account_id - phishing

## **Status:** Complete

## What's the difference between predecessor_account_id and signer_account_id ?
If contract A calls B, and B calls C, in C predecessor_account_id is B and signer_account_id is A.

## Vulnerability
A malicious contract can deceive the owner of a contract into calling a function that only the owner should be able to call.