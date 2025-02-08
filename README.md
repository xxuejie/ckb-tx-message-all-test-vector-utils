# cighash-all-test-vector-utils

A suit of utilities & sample contracts leveraging the new `CIGHASH_ALL` spec. Several notable components include:

* [crates/cighash-all-utils](./crates/sighash-all-utils): A Rust crate that performs `CIGHASH_ALL` calculation. Both on-chain and off-chain environments are supported.
* [crates/native-test-vector-generator](./crates/native-test-vector-generator): A native test vector generator for working with `CIGHASH_ALL` spec.
* [contracts/rust-assert-cighash](./contracts/rust-assert-cighash): A simple Rust-based CKB script that validates the `lock` field from the first witness(in `WitnessArgs` structure) of current script group, contains the `CIGHASH_ALL` hash for current transaction & script group, using CKB flavored blake2b hash as the hasher. Notice this is not a secure lock script, a proper one shall validate a signature calculated on the `CIGHASH_ALL` hash, not comparing the hash value directly.
* [contracts/rust-assert-cighash](./contracts/c-assert-cighash): A simple C-based CKB script that validates the `lock` field from the first witness(in `WitnessArgs` structure) of current script group, contains the `CIGHASH_ALL` hash for current transaction & script group, using CKB flavored blake2b hash as the hasher. Notice this is not a secure lock script, a proper one shall validate a signature calculated on the `CIGHASH_ALL` hash, not comparing the hash value directly.

*This project was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/cryptape/ckb-script-templates
