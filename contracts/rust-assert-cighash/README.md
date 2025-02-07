# rust-assert-cighash

This CKB script written in Rust calculates signing message following the `CIGHASH_ALL` spec using a [ckb-hash](https://docs.rs/ckb-hash/latest/ckb_hash/) hasher, it then compare the resulting message hash with content in the `lock` field of the first witness (in `WitnessArgs` structure) from the current script group. If the 2 values match, the script terminates with a success return code, otherwise a failure is generated.

*This contract was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/cryptape/ckb-script-templates
