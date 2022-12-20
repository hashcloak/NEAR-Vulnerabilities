# Unchecked Return Value

## **Status:** Complete

## Vulnerability:
The return value of external function call is not checked. If the call fails accidentally or an attacker forces the call to fail, this may cause unexpected behaviour in the subsequent program logic.
