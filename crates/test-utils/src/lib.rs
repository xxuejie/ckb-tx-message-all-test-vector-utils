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
use ckb_tx_message_all_utils::ckb_tx_message_all_from_mock_tx::{
    generate_ckb_tx_message_all_from_mock_tx, ScriptOrIndex,
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
) -> (Context, TransactionView, Vec<usize>) {
    let mut rng = StdRng::seed_from_u64(seed);

    _build_bare_tx(contract_bin, always_success_bin, &mut rng, 1, 5)
}

/// Build a bare minimal transaction with 3 - 5 input cells
/// using provided lock, and minimal data for witnesses
pub fn build_bare_tx_multiple_input_cells(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    seed: u64,
) -> (Context, TransactionView, Vec<usize>) {
    let mut rng = StdRng::seed_from_u64(seed);

    _build_bare_tx(contract_bin, always_success_bin, &mut rng, 3, 5)
}

/// Build a proper transaction with 3 - 5 input cells
/// using provided lock, witness shall also be filled with real data
pub fn build_tx_with_witness_data(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    seed: u64,
) -> (Context, TransactionView, Vec<usize>) {
    let mut rng = StdRng::seed_from_u64(seed);

    let (mut context, uncompleted_tx, indices) = _build_bare_uncompleted_tx_with_witness(
        contract_bin,
        always_success_bin,
        &mut rng,
        3,
        5,
        10,
        200,
    );

    let signed_tx = complete_and_sign_tx(&mut context, uncompleted_tx, indices[0]);

    (context, signed_tx, indices)
}

/// Build a proper transaction with 2 - 4 input cells
/// using provided lock, super large data will be used to test streaming APIs
pub fn build_tx_with_super_large_data(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    seed: u64,
) -> (Context, TransactionView, Vec<usize>) {
    let mut rng = StdRng::seed_from_u64(seed);

    let (mut context, uncompleted_tx, indices) = _build_bare_uncompleted_tx_with_witness(
        contract_bin,
        always_success_bin,
        &mut rng,
        2,
        4,
        70000,
        300000,
    );

    // Modify some input cells with large data
    {
        let modified_count = rng.gen_range(1..=uncompleted_tx.inputs().len());

        for cell_input in uncompleted_tx.inputs().into_iter().take(modified_count) {
            let data_length = rng.gen_range(70000..=300000);
            let data = random_data(&mut rng, data_length);

            {
                let pair = context
                    .cells
                    .get_mut(&cell_input.previous_output())
                    .unwrap();
                pair.1 = data;
            }
        }
    }

    let signed_tx = complete_and_sign_tx(&mut context, uncompleted_tx, indices[0]);

    (context, signed_tx, indices)
}

fn _build_bare_tx<R: Rng>(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    rng: &mut R,
    min_current_group_input_cells: usize,
    max_current_group_input_cells: usize,
) -> (Context, TransactionView, Vec<usize>) {
    let (mut context, uncompleted_tx, indices) = _build_bare_uncompleted_tx(
        contract_bin,
        always_success_bin,
        rng,
        min_current_group_input_cells,
        max_current_group_input_cells,
    );

    let signed_tx = complete_and_sign_tx(&mut context, uncompleted_tx, indices[0]);

    (context, signed_tx, indices)
}

fn _build_bare_uncompleted_tx_with_witness<R: Rng>(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    rng: &mut R,
    min_current_group_input_cells: usize,
    max_current_group_input_cells: usize,
    min_witness_length: usize,
    max_witness_length: usize,
) -> (Context, TransactionView, Vec<usize>) {
    let (context, uncompleted_tx, indices) = _build_bare_uncompleted_tx(
        contract_bin,
        always_success_bin,
        rng,
        min_current_group_input_cells,
        max_current_group_input_cells,
    );

    // Modify the tx to fill in witness data
    let modified_tx = {
        let generated_witness_count = uncompleted_tx.inputs().len() + rng.gen_range(1..=3);
        let witnesses: Vec<_> = (0..generated_witness_count)
            .map(|i| {
                if i == indices[0] {
                    let current_data = uncompleted_tx.witnesses().get(i).unwrap().raw_data();
                    let current_witness_args = WitnessArgs::from_slice(&current_data).unwrap();

                    let input_type_length = rng.gen_range(min_witness_length..=max_witness_length);
                    let input_type = random_data(rng, input_type_length);
                    let output_type_length = rng.gen_range(min_witness_length..=max_witness_length);
                    let output_type = random_data(rng, output_type_length);

                    current_witness_args
                        .as_builder()
                        .input_type(Some(input_type).pack())
                        .output_type(Some(output_type).pack())
                        .build()
                        .as_bytes()
                } else {
                    let length = rng.gen_range(min_witness_length..=max_witness_length);
                    random_data(rng, length)
                }
            })
            .map(|b| b.pack())
            .collect();

        uncompleted_tx
            .as_advanced_builder()
            .set_witnesses(witnesses)
            .build()
    };

    (context, modified_tx, indices)
}

fn _build_bare_uncompleted_tx<R: Rng>(
    contract_bin: Bytes,
    always_success_bin: Bytes,
    rng: &mut R,
    min_current_group_input_cells: usize,
    max_current_group_input_cells: usize,
) -> (Context, TransactionView, Vec<usize>) {
    assert!(min_current_group_input_cells > 0);

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
        let input = build_input_cell(&mut context, rng, &lock_script, 0, 200, 200, 100000);

        inputs.push((input, true));
    }
    for _ in 0..rng.gen_range(1..=6) {
        let input = build_input_cell(
            &mut context,
            rng,
            &always_success_script,
            0,
            150,
            150,
            20000,
        );

        inputs.push((input, false));
    }
    inputs.shuffle(rng);

    let mut outputs = vec![];
    let mut outputs_data = vec![];
    for _ in 0..rng.gen_range(3..=6) {
        let (output, data) = build_output_cell(rng, &lock_script, 0, 300, 2000, 30000);

        outputs.push(output);
        outputs_data.push(data);
    }

    // Prepare just enough witness
    let indices: Vec<_> = inputs
        .iter()
        .enumerate()
        .filter(|(_, (_, f))| *f)
        .map(|(i, _)| i)
        .collect();
    let first_witness_index = indices[0];
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

    (context, signed_tx, indices)
}

fn complete_and_sign_tx(
    context: &mut Context,
    uncompleted_tx: TransactionView,
    first_witness_index: usize,
) -> TransactionView {
    let unsigned_tx = context.complete_tx(uncompleted_tx);

    let ckb_tx_message = {
        let unsigned_mock_tx = context.dump_tx(&unsigned_tx).expect("dump tx");
        let mut hasher = Hasher::default();
        generate_ckb_tx_message_all_from_mock_tx(
            &unsigned_mock_tx.into(),
            ScriptOrIndex::Index(first_witness_index),
            &mut hasher,
        )
        .expect("generate ckb tx message all");
        hasher.hash()
    };

    // Use ckb_tx_message to replace the placeholder part in unsigned transaction
    let mut witnesses: Vec<_> = unsigned_tx.witnesses().into_iter().collect();
    let first_witness =
        WitnessArgs::from_slice(&witnesses[first_witness_index].raw_data()).unwrap();
    witnesses[first_witness_index] = first_witness
        .as_builder()
        .lock(Some(Bytes::from(ckb_tx_message.to_vec())).pack())
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
    let data = random_data(rng, data_length);

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

fn random_data<R: Rng>(rng: &mut R, length: usize) -> Bytes {
    let mut data = vec![0u8; length];
    rng.fill(&mut data[..]);
    data.into()
}
