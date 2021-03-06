#![deny(
    unused_qualifications,
    missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces
)]

#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

use std::prelude::v1::*;

// *merkle* implements a Merkle Tree in Rust.

extern crate crypto;

#[cfg(feature = "serialization-protobuf")]
extern crate protobuf;

mod merkletree;
pub use merkletree::MerkleTree;

mod proof;
pub use proof::Proof;

mod hashutils;
pub use hashutils::{Hashable, HashUtils};

mod tree;
pub use tree::{ LeavesIterator, LeavesIntoIterator };

#[cfg(feature = "serialization-protobuf")]
#[allow(unused_qualifications)]
mod proto;

#[cfg(test)]
mod tests;

