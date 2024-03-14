#![no_std]

extern crate alloc;

pub use shielder::wasm::*;
#[cfg(feature = "multithreading")]
pub use wasm_bindgen_rayon::init_thread_pool;
