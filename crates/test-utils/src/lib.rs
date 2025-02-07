use cighash_all_utils::cighash_all_from_mock_tx::{
    generate_cighash_all_from_mock_tx, ScriptOrIndex,
};
use ckb_testtool::{
    ckb_hash::{new_blake2b, Blake2b},
    ckb_types::{
        bytes::Bytes,
        core::{TransactionBuilder, TransactionView},
        packed::*,
        prelude::*,
    },
    context::Context,
};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::io;

pub struct Hasher(Blake2b);

impl Hasher {
    pub fn hash(self) -> [u8; 32] {
        let mut result = [0u8; 32];
        self.0.finalize(&mut result);
        result
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Hasher(new_blake2b())
    }
}

impl io::Write for Hasher {
    fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
        self.0.update(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

/// Build a bare minimal transaction with 1 - 5 input cells
/// using provided lock, and minimal data for witnesses
pub fn build_bare_tx(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    seed: u64,
) -> (Context, TransactionView) {
    _build_bare_tx(contract_bin, always_success_bin, seed, 1, 5)
}

/// Build a bare minimal transaction with 3 - 5 input cells
/// using provided lock, and minimal data for witnesses
pub fn build_bare_tx_multiple_input_cells(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    seed: u64,
) -> (Context, TransactionView) {
    _build_bare_tx(contract_bin, always_success_bin, seed, 3, 5)
}

fn _build_bare_tx(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    seed: u64,
    min_current_group_input_cells: usize,
    max_current_group_input_cells: usize,
) -> (Context, TransactionView) {
    assert!(min_current_group_input_cells > 0);

    // Setup rng
    let mut rng = StdRng::seed_from_u64(seed);

    let mut context = Context::new_with_deterministic_rng();
    let out_point = context.deploy_cell(contract_bin);
    let always_success_out_point = context.deploy_cell(always_success_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::new())
        .expect("script");
    let always_success_script = context
        .build_script(&always_success_out_point, Bytes::new())
        .expect("script");

    // prepare cells
    let mut inputs = vec![];
    for _ in 0..rng.gen_range(min_current_group_input_cells..=max_current_group_input_cells) {
        let input = build_input_cell(&mut context, &mut rng, &lock_script, 0, 200, 200, 100000);

        inputs.push((input, true));
    }
    for _ in 0..rng.gen_range(1..=8) {
        let input = build_input_cell(
            &mut context,
            &mut rng,
            &always_success_script,
            0,
            150,
            150,
            20000,
        );

        inputs.push((input, false));
    }
    inputs.shuffle(&mut rng);

    let mut outputs = vec![];
    let mut outputs_data = vec![];
    for _ in 0..rng.gen_range(3..=6) {
        let (output, data) = build_output_cell(&mut rng, &lock_script, 0, 300, 2000, 30000);

        outputs.push(output);
        outputs_data.push(data);
    }

    // Prepare just enough witness
    let first_witness_index = inputs.iter().position(|(_, f)| *f).unwrap();
    let mut witnesses = vec![Bytes::new(); first_witness_index + 1];
    witnesses[first_witness_index] = WitnessArgs::new_builder()
        .lock(Some(Bytes::from(vec![0u8; 32])).pack())
        .build()
        .as_bytes();

    // Build transaction
    let uncompleted_tx = TransactionBuilder::default()
        .inputs(inputs.into_iter().map(|(i, _)| i))
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .witnesses(witnesses.pack())
        .build();
    let signed_tx = complete_and_sign_tx(&mut context, uncompleted_tx, first_witness_index);

    (context, signed_tx)
}

fn complete_and_sign_tx(
    context: &mut Context,
    uncompleted_tx: TransactionView,
    first_witness_index: usize,
) -> TransactionView {
    let unsigned_tx = context.complete_tx(uncompleted_tx);

    let cighash = {
        let unsigned_mock_tx = context.dump_tx(&unsigned_tx).expect("dump tx");
        let mut hasher = Hasher::default();
        generate_cighash_all_from_mock_tx(
            &unsigned_mock_tx.into(),
            ScriptOrIndex::Index(first_witness_index),
            &mut hasher,
        )
        .expect("generate cighash all");
        hasher.hash()
    };

    // Use cighash to replace the placeholder part in unsigned transaction
    let mut witnesses: Vec<_> = unsigned_tx.witnesses().into_iter().collect();
    witnesses[first_witness_index] = WitnessArgs::new_builder()
        .lock(Some(Bytes::from(cighash.to_vec())).pack())
        .build()
        .as_bytes()
        .pack();
    unsigned_tx
        .as_advanced_builder()
        .set_witnesses(witnesses)
        .build()
}

fn build_input_cell<R: Rng>(
    context: &mut Context,
    rng: &mut R,
    script: &Script,
    min_data_length: usize,
    max_data_length: usize,
    min_capacity_bytes: usize,
    max_capacity_bytes: usize,
) -> CellInput {
    let data_length = rng.gen_range(min_data_length..=max_data_length);
    let mut data = vec![0u8; data_length];
    rng.fill(&mut data[..]);
    let data: Bytes = data.into();

    let capacity = (data_length + rng.gen_range(min_capacity_bytes..=max_capacity_bytes))
        * 100_000_000
        + rng.gen_range(0..100_000_000);

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity((capacity as u64).pack())
            .lock(script.clone())
            .build(),
        data,
    );
    CellInput::new_builder()
        .previous_output(input_out_point)
        .build()
}

fn build_output_cell<R: Rng>(
    rng: &mut R,
    script: &Script,
    min_data_length: usize,
    max_data_length: usize,
    min_capacity_bytes: usize,
    max_capacity_bytes: usize,
) -> (CellOutput, Bytes) {
    let data_length = rng.gen_range(min_data_length..=max_data_length);
    let mut data = vec![0u8; data_length];
    rng.fill(&mut data[..]);
    let data: Bytes = data.into();

    let capacity = (data_length + rng.gen_range(min_capacity_bytes..=max_capacity_bytes))
        * 100_000_000
        + rng.gen_range(0..100_000_000);

    (
        CellOutput::new_builder()
            .capacity((capacity as u64).pack())
            .lock(script.clone())
            .build(),
        data,
    )
}
