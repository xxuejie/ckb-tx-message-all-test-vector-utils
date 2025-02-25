use crate::Loader;
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;
use proptest::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use test_utils::*;

fn _test_valid_bare_tx(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (context, tx, _) = build_bare_tx(contract_bin, success_bin, seed);

    // run
    let cycles = context
        .verify_tx(&tx, 10_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

proptest! {
    #[test]
    fn test_rust_assert_ckb_tx_message_on_valid_bare_tx(seed: u64) {
        _test_valid_bare_tx("rust-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_c_assert_ckb_tx_message_on_valid_bare_tx(seed: u64) {
        _test_valid_bare_tx("c-assert-ckb-tx-message-all", seed);
    }

}

fn _test_valid_tx_with_witness(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (context, tx, _) = build_tx_with_witness_data(contract_bin, success_bin, seed);

    // run
    let cycles = context
        .verify_tx(&tx, 100_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

proptest! {
    #[test]
    fn test_c_assert_ckb_tx_message_on_valid_tx_with_witness(seed: u64) {
        _test_valid_tx_with_witness("c-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_rust_assert_ckb_tx_message_on_valid_tx_with_witness(seed: u64) {
        _test_valid_tx_with_witness("rust-assert-ckb-tx-message-all", seed);
    }
}

fn _test_valid_tx_with_super_large_data(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (context, tx, _) = build_tx_with_super_large_data(contract_bin, success_bin, seed);

    // run
    let cycles = context
        .verify_tx(&tx, 100_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

proptest! {
    // Tests with large data can be quite time-consuming, we are limiting the proptest
    // runs here.
    #![proptest_config(ProptestConfig {
        cases: 30, .. ProptestConfig::default()
    })]

    #[test]
    fn test_c_assert_ckb_tx_message_on_valid_tx_with_super_large_data(seed: u64) {
        _test_valid_tx_with_super_large_data("c-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_rust_assert_ckb_tx_message_on_valid_tx_with_super_large_data(seed: u64) {
        _test_valid_tx_with_super_large_data("rust-assert-ckb-tx-message-all", seed);
    }
}

fn _test_unsigned_input_amount_bare_tx(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (mut context, tx, _) = build_bare_tx(contract_bin, success_bin, seed);

    // Modify the CKBytes of one particular input cell
    {
        let op = tx.inputs().get(0).unwrap().previous_output();
        let pair = context.cells.get_mut(&op).unwrap();

        let old_capacity: u64 = pair.0.capacity().unpack();
        let new_output = pair
            .0
            .clone()
            .as_builder()
            .capacity((old_capacity + 1).pack())
            .build();

        pair.0 = new_output;
    }

    // run to a failure
    context.verify_tx(&tx, 10_000_000).unwrap_err();
}

proptest! {
    #[test]
    fn test_rust_assert_ckb_tx_message_on_unsigned_input_amount_bare_tx(seed: u64) {
        _test_unsigned_input_amount_bare_tx("rust-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_c_assert_ckb_tx_message_on_unsigned_input_amount_bare_tx(seed: u64) {
        _test_unsigned_input_amount_bare_tx("c-assert-ckb-tx-message-all", seed);
    }

}

fn _test_unsigned_input_cell_data_bare_tx(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (mut context, tx, _) = build_bare_tx(contract_bin, success_bin, seed);

    // Modify the cell data of one particular input cell
    {
        let op = tx.inputs().get(0).unwrap().previous_output();
        let pair = context.cells.get_mut(&op).unwrap();

        let mut data = pair.1.clone().to_vec();
        if data.len() > 1 {
            data.pop();
        } else {
            data.push(42);
        }

        pair.1 = data.into();
    }

    // run to a failure
    context.verify_tx(&tx, 10_000_000).unwrap_err();
}

proptest! {
    #[test]
    fn test_rust_assert_ckb_tx_message_on_unsigned_input_cell_data_bare_tx(seed: u64) {
        _test_unsigned_input_cell_data_bare_tx("rust-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_c_assert_ckb_tx_message_on_unsigned_input_cell_data_bare_tx(seed: u64) {
        _test_unsigned_input_cell_data_bare_tx("c-assert-ckb-tx-message-all", seed);
    }
}

fn _test_unsigned_tx_data_bare_tx(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (context, tx, _) = build_bare_tx(contract_bin, success_bin, seed);

    // Modify one particular output cell so tx changes
    let tx = {
        tx.as_advanced_builder()
            .output(CellOutput::new_builder().build())
            .build()
    };

    // run to a failure
    context.verify_tx(&tx, 10_000_000).unwrap_err();
}

proptest! {
    #[test]
    fn test_rust_assert_ckb_tx_message_on_unsigned_tx_data_bare_tx(seed: u64) {
        _test_unsigned_tx_data_bare_tx("rust-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_c_assert_ckb_tx_message_on_unsigned_tx_data_bare_tx(seed: u64) {
        _test_unsigned_tx_data_bare_tx("c-assert-ckb-tx-message-all", seed);
    }
}

fn _test_ckb_tx_message_on_appended_witness_bare_tx(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (context, tx, _) = build_bare_tx_multiple_input_cells(contract_bin, success_bin, seed);

    // Add more witnesses than originally planned
    let tx = {
        let input_count = tx.inputs().len();
        let witness_count = tx.witnesses().len();

        let mut builder = tx.as_advanced_builder();
        assert!(witness_count < input_count);
        for _ in 0..(input_count - witness_count) {
            builder = builder.witness(Bytes::new().pack());
        }
        builder.build()
    };

    // run to a failure
    context.verify_tx(&tx, 10_000_000).unwrap_err();
}

proptest! {
    #[test]
    fn test_rust_assert_ckb_tx_message_on_appended_witness_bare_tx(seed: u64) {
        _test_ckb_tx_message_on_appended_witness_bare_tx("rust-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_c_assert_ckb_tx_message_on_appended_witness_bare_tx(seed: u64) {
        _test_ckb_tx_message_on_appended_witness_bare_tx("c-assert-ckb-tx-message-all", seed);
    }
}

fn _test_ckb_tx_message_on_invalid_witness_bare_tx(contract_name: &str, seed: u64) {
    let contract_bin: Bytes = Loader::default().load_binary(contract_name);
    let success_bin: Bytes = Loader::default().load_binary("always-success");

    let (context, tx, indices) = build_bare_tx(contract_bin, success_bin, seed);

    // Flip one of the first 128 bits(16 bytes) of the last witness,
    // which contains a WitnessArgs structure
    let tx = {
        let mut witnesses: Vec<_> = tx.witnesses().into_iter().collect();
        let mut last_witness = witnesses[indices[0]].raw_data().to_vec();

        let mut rng = StdRng::seed_from_u64(seed.wrapping_add(1));
        let byte_index = rng.gen_range(0..16);
        let bit_index = rng.gen_range(0..8);

        last_witness[byte_index] ^= 1 << bit_index;

        let last_witness: Bytes = last_witness.into();
        witnesses[indices[0]] = last_witness.pack();

        tx.as_advanced_builder().set_witnesses(witnesses).build()
    };

    // run to a failure
    context.verify_tx(&tx, 10_000_000).unwrap_err();
}

proptest! {
    #[test]
    fn test_rust_assert_ckb_tx_message_on_invalid_witness_bare_tx(seed: u64) {
        _test_ckb_tx_message_on_invalid_witness_bare_tx("rust-assert-ckb-tx-message-all", seed);
    }

    #[test]
    fn test_c_assert_ckb_tx_message_on_invalid_witness_bare_tx(seed: u64) {
        _test_ckb_tx_message_on_invalid_witness_bare_tx("c-assert-ckb-tx-message-all", seed);
    }
}

// generated unit test for contract always_success
#[test]
fn test_always_success() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("always-success");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::from(vec![42]))
        .expect("script");

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, 10_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}
