# Re-Entrancy

## **Status:** Incomplete

## Vulnerability:
Between a cross-contract call and its callback any method of your contract can be executed. Not taking this into account anyone can result in exploits.

## Reference: [re-entrancy](https://docs.near.org/develop/contracts/security/callbacks)

### Blocker:
Unable to call withdraw function in between deposit and it's callback function's call to reproduce the re-entrancy bug. Need to figure out a way if the code's logic have some mistake or there is any other requirement to call the withdraw function on time.
