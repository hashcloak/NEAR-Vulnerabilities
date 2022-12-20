# Arithmetic Overflow

## **Status:** Complete

## Vulnerability
Integers in Rust overflow / underflow without any errors, If we don't use `overflow-checks = true` in `Cargo.toml`.