# native-test-vector-generator

A utility to generate test vectors for `CKB_TX_MESSAGE_ALL` standard.

## Usage

Use the following command to setup the tool:

```bash
$ git clone --recursive https://github.com/xxuejie/ckb-tx-message-all-test-vector-utils
$ cd ckb-tx-message-all-test-vector-utils
$ make build
```

The most commonly used command generates a series of test vectors in one batch:

```bash
$ ./target/release/native-test-vector-generator --output ./test-vector1
Seed: 1738994393865210900
$ ls -lh test-vector1
total 19M
-rw-rw-r-- 1 user   64 Feb  8 13:59 bare-tx-batch1.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 bare-tx-batch1.indices
-rw-rw-r-- 1 user 171K Feb  8 13:59 bare-tx-batch1.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 bare-tx-batch2.hash
-rw-rw-r-- 1 user   12 Feb  8 13:59 bare-tx-batch2.indices
-rw-rw-r-- 1 user 168K Feb  8 13:59 bare-tx-batch2.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 bare-tx-batch3.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 bare-tx-batch3.indices
-rw-rw-r-- 1 user 167K Feb  8 13:59 bare-tx-batch3.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 bare-tx-batch4.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 bare-tx-batch4.indices
-rw-rw-r-- 1 user 172K Feb  8 13:59 bare-tx-batch4.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 bare-tx-batch5.hash
-rw-rw-r-- 1 user   12 Feb  8 13:59 bare-tx-batch5.indices
-rw-rw-r-- 1 user 168K Feb  8 13:59 bare-tx-batch5.json
-rw-rw-r-- 1 user   22 Feb  8 13:59 invalid-witness-tx-batch1.indices
-rw-rw-r-- 1 user 174K Feb  8 13:59 invalid-witness-tx-batch1.json
-rw-rw-r-- 1 user   27 Feb  8 13:59 invalid-witness-tx-batch2.indices
-rw-rw-r-- 1 user 171K Feb  8 13:59 invalid-witness-tx-batch2.json
-rw-rw-r-- 1 user   27 Feb  8 13:59 invalid-witness-tx-batch3.indices
-rw-rw-r-- 1 user 176K Feb  8 13:59 invalid-witness-tx-batch3.json
-rw-rw-r-- 1 user   28 Feb  8 13:59 invalid-witness-tx-batch4.indices
-rw-rw-r-- 1 user 175K Feb  8 13:59 invalid-witness-tx-batch4.json
-rw-rw-r-- 1 user   17 Feb  8 13:59 invalid-witness-tx-batch5.indices
-rw-rw-r-- 1 user 171K Feb  8 13:59 invalid-witness-tx-batch5.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 large-data-tx-batch1.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 large-data-tx-batch1.indices
-rw-rw-r-- 1 user 6.0M Feb  8 13:59 large-data-tx-batch1.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 large-data-tx-batch2.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 large-data-tx-batch2.indices
-rw-rw-r-- 1 user 4.1M Feb  8 13:59 large-data-tx-batch2.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 large-data-tx-batch3.hash
-rw-rw-r-- 1 user   12 Feb  8 13:59 large-data-tx-batch3.indices
-rw-rw-r-- 1 user 4.0M Feb  8 13:59 large-data-tx-batch3.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 multiple-input-tx-batch1.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 multiple-input-tx-batch1.indices
-rw-rw-r-- 1 user 170K Feb  8 13:59 multiple-input-tx-batch1.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 multiple-input-tx-batch2.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 multiple-input-tx-batch2.indices
-rw-rw-r-- 1 user 170K Feb  8 13:59 multiple-input-tx-batch2.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 multiple-input-tx-batch3.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 multiple-input-tx-batch3.indices
-rw-rw-r-- 1 user 171K Feb  8 13:59 multiple-input-tx-batch3.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 multiple-input-tx-batch4.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 multiple-input-tx-batch4.indices
-rw-rw-r-- 1 user 168K Feb  8 13:59 multiple-input-tx-batch4.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 multiple-input-tx-batch5.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 multiple-input-tx-batch5.indices
-rw-rw-r-- 1 user 168K Feb  8 13:59 multiple-input-tx-batch5.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch1.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 witness-tx-batch1.indices
-rw-rw-r-- 1 user 173K Feb  8 13:59 witness-tx-batch1.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch10.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 witness-tx-batch10.indices
-rw-rw-r-- 1 user 169K Feb  8 13:59 witness-tx-batch10.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch2.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 witness-tx-batch2.indices
-rw-rw-r-- 1 user 173K Feb  8 13:59 witness-tx-batch2.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch3.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 witness-tx-batch3.indices
-rw-rw-r-- 1 user 171K Feb  8 13:59 witness-tx-batch3.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch4.hash
-rw-rw-r-- 1 user   28 Feb  8 13:59 witness-tx-batch4.indices
-rw-rw-r-- 1 user 175K Feb  8 13:59 witness-tx-batch4.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch5.hash
-rw-rw-r-- 1 user   22 Feb  8 13:59 witness-tx-batch5.indices
-rw-rw-r-- 1 user 172K Feb  8 13:59 witness-tx-batch5.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch6.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 witness-tx-batch6.indices
-rw-rw-r-- 1 user 170K Feb  8 13:59 witness-tx-batch6.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch7.hash
-rw-rw-r-- 1 user   17 Feb  8 13:59 witness-tx-batch7.indices
-rw-rw-r-- 1 user 168K Feb  8 13:59 witness-tx-batch7.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch8.hash
-rw-rw-r-- 1 user   27 Feb  8 13:59 witness-tx-batch8.indices
-rw-rw-r-- 1 user 177K Feb  8 13:59 witness-tx-batch8.json
-rw-rw-r-- 1 user   64 Feb  8 13:59 witness-tx-batch9.hash
-rw-rw-r-- 1 user   27 Feb  8 13:59 witness-tx-batch9.indices
-rw-rw-r-- 1 user 172K Feb  8 13:59 witness-tx-batch9.json
```

Each different file name minus the file extension part represents a different test case. For each test case, 2 or 3 files will be generated:

* `.json` suffix: a JSON file containing mock transaction in a format that will be accepted by [ckb-debugger](https://github.com/nervosnetwork/ckb-standalone-debugger).
* `.indices` suffix: a JSON file containing indices for input cells that use a `CKB_TX_MESSAGE_ALL` validating lock
* `.hash` suffix: an optional file, in the case a `CKB_TX_MESSAGE_ALL` hash could be generated, this contains a 32-byte hash in hex notation, which is the `CKB_TX_MESSAGE_ALL` generated from the JSON tx file of the same name, using the indices file of the same name as the specified script group, and using ckb flavored blake2b hash function as the hasher. In case a `CKB_TX_MESSAGE_ALL` hash could not be generated(e.g., the first witness in current script group is not WitnessArgs structure), this file will be missing.

For example, `witness-tx-batch10.hash` contains the `CKB_TX_MESSAGE_ALL` hash generated for the tx file `witness-tx-batch10.json`, using input cells denoted in `witness-tx-batch10.indices` as the current script group. CKB flavored blake2b hash(meaning blake2b's personalization is set to `ckb-default-hash`) is used to calculate the final hash.

On the other hand, `invalid-witness-tx-batch3.json` represents a different CKB transaction, which has no valid `CKB_TX_MESSAGE_ALL` hash using inputs cells denoted in `invalid-witness-tx-batch3.indices` as the current script group.

One can also specify the seed to use for deterministic generation:

```bash
$ ./target/release/native-test-vector-generator --output ./test-vector2 --seed 3
Seed: 3
```

It's possible to generate a single test case as well:

```bash
$ ./target/release/native-test-vector-generator --output ./test-vector3 --mode witness --seed 14
Seed: 14
$ ls -lh test-vector3
total 184K
-rw-rw-r-- 1 user   64 Feb  8 14:07 witness-tx-from-seed-14.hash
-rw-rw-r-- 1 user   22 Feb  8 14:07 witness-tx-from-seed-14.indices
-rw-rw-r-- 1 user 175K Feb  8 14:07 witness-tx-from-seed-14.json
```

Please use `--help` if you want to learn about the details of the generator command.
