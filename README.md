# ckb-tx-message-all-test-vector-utils

A suit of utilities & sample contracts leveraging the new `CKB_TX_MESSAGE_ALL` spec. Several notable components include:

* [crates/ckb-tx-message-all-utils](./crates/ckb-tx-message-all-utils): A Rust crate that performs `CKB_TX_MESSAGE_ALL` calculation. Both on-chain and off-chain environments are supported.
* [crates/native-test-vector-generator](./crates/native-test-vector-generator): A native test vector generator for working with `CKB_TX_MESSAGE_ALL` spec.
* [contracts/rust-assert-ckb-tx-message-all](./contracts/rust-assert-ckb-tx-message-all): A simple Rust-based CKB script that validates the `lock` field from the first witness(in `WitnessArgs` structure) of current script group, contains the `CKB_TX_MESSAGE_ALL` hash for current transaction & script group, using CKB flavored blake2b hash as the hasher. Notice this is not a secure lock script, a proper one shall validate a signature calculated on the `CKB_TX_MESSAGE_ALL` hash, not comparing the hash value directly.
* [contracts/rust-assert-ckb-tx-message-all](./contracts/c-assert-ckb-tx-message-all): A simple C-based CKB script that validates the `lock` field from the first witness(in `WitnessArgs` structure) of current script group, contains the `CKB_TX_MESSAGE_ALL` hash for current transaction & script group, using CKB flavored blake2b hash as the hasher. Notice this is not a secure lock script, a proper one shall validate a signature calculated on the `CKB_TX_MESSAGE_ALL` hash, not comparing the hash value directly.

*This project was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/cryptape/ckb-script-templates
