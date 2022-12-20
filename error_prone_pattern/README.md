# Error Prone Patterns

## **Status:** Incomplete

## [Bug 1](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern/Bug_1):

### Vulnerability:
We should not replace any collections without clearing state, this will reset any metadata, such as the number of elements, leading to bugs. If you replace the collection with something with a different prefix, it will be functional, but you will lose any previous data and the old values will not be removed from storage.

## [Bug 2](https://github.com/hashcloak/NEAR-Vulnerabilities/tree/main/error_prone_pattern/Bug_2):

### Vulnerability:
We should not use the same prefix as another collection or there will be unexpected side effects.

### Blocker:
Whenever using same prefix the contracts state are changed in  a state that's it will not work any more and will show some issues regarding contracts state.