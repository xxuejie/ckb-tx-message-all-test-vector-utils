use ckb_testtool::{
    ckb_types::{bytes::Bytes, core::TransactionView, prelude::*},
    context::Context,
};
use ckb_tx_message_all_utils::ckb_tx_message_all_from_mock_tx::{
    generate_ckb_tx_message_all_from_mock_tx, ScriptOrIndex,
};
use clap::{Parser, ValueEnum};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fs;
use std::path::Path;
use test_utils::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Generate all vectors
    All,
    /// Generate bare tx with just enough witness
    Bare,
    /// Generate bare tx with more than one input cell in current script group
    MultipleInput,
    /// Generate tx with witness data
    Witness,
    /// Generate tx with large data
    LargeData,
    /// Generate invalid tx with first witness that is not WitnessArgs
    InvalidWitness,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Generation mode
    #[arg(long, value_enum, default_value_t = Mode::All)]
    mode: Mode,

    /// Seed for random number generator
    #[arg(long)]
    seed: Option<u64>,

    /// Output folder
    #[arg(long)]
    output: String,

    /// CKB_TX_MESSAGE contract
    #[arg(long, default_value = "./build/release/rust-assert-ckb-tx-message-all")]
    contract: String,

    /// Always success contract
    #[arg(long, default_value = "./build/release/always-success")]
    always_success: String,
}

fn main() {
    let cli = Cli::parse();

    let seed = match cli.seed {
        Some(seed) => seed,
        None => std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    };
    println!("Seed: {}", seed);

    fs::create_dir_all(&cli.output).expect("mkdir");

    match cli.mode {
        Mode::All => {
            let mut rng = StdRng::seed_from_u64(seed);

            for i in 1..=5 {
                save_bare_tx(&cli, rng.gen(), &format!("-batch{}", i));
            }
            for i in 1..=5 {
                save_multiple_input_tx(&cli, rng.gen(), &format!("-batch{}", i));
            }
            for i in 1..=10 {
                save_witness_tx(&cli, rng.gen(), &format!("-batch{}", i));
            }
            for i in 1..=5 {
                save_invalid_witness_tx(&cli, rng.gen(), &format!("-batch{}", i));
            }
            for i in 1..=3 {
                save_large_data_tx(&cli, rng.gen(), &format!("-batch{}", i));
            }
        }
        Mode::Bare => {
            save_bare_tx(&cli, seed, &format!("-from-seed-{}", seed));
        }
        Mode::MultipleInput => {
            save_multiple_input_tx(&cli, seed, &format!("-from-seed-{}", seed));
        }
        Mode::Witness => {
            save_witness_tx(&cli, seed, &format!("-from-seed-{}", seed));
        }
        Mode::LargeData => {
            save_large_data_tx(&cli, seed, &format!("-from-seed-{}", seed));
        }
        Mode::InvalidWitness => {
            save_invalid_witness_tx(&cli, seed, &format!("-from-seed-{}", seed));
        }
    }
}

fn save_bare_tx(cli: &Cli, seed: u64, suffix: &str) {
    let contract_bin: Bytes = fs::read(&cli.contract).expect("read").into();
    let always_success_bin: Bytes = fs::read(&cli.always_success).expect("read").into();
    let path = Path::new(&cli.output).join(format!("bare-tx{}", suffix));

    let (context, tx, indices) = build_bare_tx(contract_bin, always_success_bin, seed);
    save_tx(context, tx, indices, &path);
}

fn save_multiple_input_tx(cli: &Cli, seed: u64, suffix: &str) {
    let contract_bin: Bytes = fs::read(&cli.contract).expect("read").into();
    let always_success_bin: Bytes = fs::read(&cli.always_success).expect("read").into();
    let path = Path::new(&cli.output).join(format!("multiple-input-tx{}", suffix));

    let (context, tx, indices) =
        build_bare_tx_multiple_input_cells(contract_bin, always_success_bin, seed);
    save_tx(context, tx, indices, &path);
}

fn save_witness_tx(cli: &Cli, seed: u64, suffix: &str) {
    let contract_bin: Bytes = fs::read(&cli.contract).expect("read").into();
    let always_success_bin: Bytes = fs::read(&cli.always_success).expect("read").into();
    let path = Path::new(&cli.output).join(format!("witness-tx{}", suffix));

    let (context, tx, indices) = build_tx_with_witness_data(contract_bin, always_success_bin, seed);
    save_tx(context, tx, indices, &path);
}

fn save_invalid_witness_tx(cli: &Cli, seed: u64, suffix: &str) {
    let contract_bin: Bytes = fs::read(&cli.contract).expect("read").into();
    let always_success_bin: Bytes = fs::read(&cli.always_success).expect("read").into();
    let path = Path::new(&cli.output).join(format!("invalid-witness-tx{}", suffix));

    let (context, tx, indices) = build_tx_with_witness_data(contract_bin, always_success_bin, seed);

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

    save_tx(context, tx, indices, &path);
}

fn save_large_data_tx(cli: &Cli, seed: u64, suffix: &str) {
    let contract_bin: Bytes = fs::read(&cli.contract).expect("read").into();
    let always_success_bin: Bytes = fs::read(&cli.always_success).expect("read").into();
    let path = Path::new(&cli.output).join(format!("large-data-tx{}", suffix));

    let (context, tx, indices) =
        build_tx_with_super_large_data(contract_bin, always_success_bin, seed);
    save_tx(context, tx, indices, &path);
}

fn save_tx(context: Context, tx: TransactionView, indices: Vec<usize>, path: &Path) {
    let path = path.to_str().expect("os str");

    let mock_tx = context.dump_tx(&tx).expect("dump tx");
    // Save tx file
    fs::write(
        format!("{}.json", path),
        serde_json::to_string_pretty(&mock_tx).expect("to json"),
    )
    .expect("write tx file");
    // Save indices
    fs::write(
        format!("{}.indices", path),
        serde_json::to_string_pretty(&indices).expect("to json"),
    )
    .expect("write index file");
    // Save message if possible
    {
        let mut hasher = Hasher::default();
        if generate_ckb_tx_message_all_from_mock_tx(
            &mock_tx.clone().into(),
            ScriptOrIndex::Index(indices[0]),
            &mut hasher,
        )
        .is_ok()
        {
            let hash: Bytes = hasher.hash().to_vec().into();
            fs::write(format!("{}.hash", path), format!("{:x}", hash)).expect("write hash");
        }
    }
}
