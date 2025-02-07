#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
pub mod cighash_all_from_mock_tx;
pub mod cighash_all_in_ckb_vm;
