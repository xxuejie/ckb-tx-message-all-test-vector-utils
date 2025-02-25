#![cfg_attr(not(any(feature = "native-simulator", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "native-simulator", test)))]
// A large heap is required if we want to load the witness as a whole
ckb_std::default_alloc!(16384, 2097152, 64);

use ckb_gen_types::{packed::WitnessArgsReader, prelude::*};
use ckb_hash::{new_blake2b, Blake2b};
use ckb_rust_std::io;
use ckb_std::{ckb_constants::Source, high_level};
use ckb_tx_message_all_utils::ckb_tx_message_all_in_ckb_vm::generate_ckb_tx_message_all;

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

pub fn program_entry() -> i8 {
    let mut hasher = Hasher::default();
    if let Err(e) = generate_ckb_tx_message_all(&mut hasher) {
        ckb_std::debug!("Generate CKB_TX_MESSAGE_ALL encounters error: {:?}", e);
        return 99;
    }
    let hash = hasher.hash();

    let first_witness_data =
        high_level::load_witness(0, Source::GroupInput).expect("load first witness data");
    let first_witness = WitnessArgsReader::from_slice(&first_witness_data)
        .expect("first witness is not WitnessArgs");

    let lock_data = first_witness
        .lock()
        .to_opt()
        .expect("lock is empty")
        .raw_data();
    assert_eq!(lock_data, hash);

    0
}
