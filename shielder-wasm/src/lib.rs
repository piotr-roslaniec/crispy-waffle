#![no_std]

extern crate alloc;

pub use shielder::wasm::*;
#[cfg(feature = "multithreading")]
pub use wasm_bindgen_rayon::init_thread_pool;

pub mod wasm_test {
    use alloc::vec;

    use shielder::MyCircuit;

    pub fn wasm_sanity_check() {
        let my_circuit = MyCircuit::new(8);
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![5, 6, 7, 8, 1, 2, 3, 4];
        let proof = my_circuit.prove(&a, &b);
        assert!(!proof.is_empty());
    }
}
