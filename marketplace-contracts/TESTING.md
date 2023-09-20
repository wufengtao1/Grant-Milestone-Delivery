# Testing Guide for Arschisinal Contracts

Welcome! This guide will help you navigate and test the "Archisinal Contracts" which are designed for a marketplace using ink! smart contract library!


Here's a brief breakdown to get you started.

> For more details, check out the [README](../README.md) for the Archisinal Contracts.

## 📂 Folder Breakdown
Think of the structure as a series of folders:

- **artifacts**: Think of these as 'instruction manuals' for the contracts.
- **contracts**: The heart of it all, the actual contracts.
    - For instance, `account_manager` handles user accounts, while `marketplace` is like a digital shop for NFTs.
- **deploy**: Scripts to help you set things up.
- **impls**: The working mechanisms behind our contracts.
- **test**: Where all testing magic happens.
    - You'll find different test types: from the all-inclusive `e2e` (end-to-end) tests, `security` and `performance`.
- **traits & events**: Special features for the contracts.
- **typechain-generated**: Automated files used with Polkadot.
- **utils**: Extra tools to help the contracts function smoothly.

## 🧪 How to Test

Follow these steps:

> **Note:** You'll need to have [Rust](https://www.rust-lang.org/tools/install) and [Node.js](https://nodejs.org/en/download/) installed.
> 
> **! Note:** You'll also need to have the [contracts-node](https://github.com/paritytech/substrate-contracts-node) installed, with version 'v0.30.0',
> Rust version: 1.68

1. **Ink-E2E Tests** 
> For now, only arch_nft has ink-e2e tests
   ```bash
   cd contracts/contract-to-test
   cargo test --features e2e-tests
   ```

2. **Comprehensive Testing**
   
   This includes E2E, Security, and Performance tests
    - E2E tests - used to test the contracts in a real environment
    - Security tests - used to test the contracts for security vulnerabilities
    - Performance tests - used to test the contracts for performance issues (e.g. gas usage)

    2.1 First, set things up:
      ```bash
      yarn
      ```
    2.2 Next, let's run all tests:
      ```bash
      yarn test:full
      ```
    2.3 If you've got the contracts-node running:
      ```bash
      yarn test:full-node
      ```

4. **Linting & Formatting**
    3.1 Check for style & coding issues:
      ```bash
      yarn lint
      cargo +nightly fmt --all -- --check
      cargo +nightly clippy --all-targets --all-features -- -D warnings
      ``` 
    3.2 Correct any formatting issues:
      ```bash
      yarn lint:fix
      cargo +nightly fmt --all
      ```

5. **Unit & Documentation Testing**
   ```bash
   cargo test
   ```
   
## P.S 

If you want to single-test some features you can use the following structure:
```bash
yarn test:mocha:<feature><?-node>
```
For example:
```bash
yarn test:mocha:marketplace
```
or
```bash
yarn test:mocha:marketplace-node
```

Happy testing! 🚀
