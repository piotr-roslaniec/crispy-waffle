#![warn(rust_2018_idioms)]

// TODO: Hide WASM specific code behind a feature flag

// #[cfg(feature = "bindings-wasm")]
extern crate alloc;

// #[cfg(feature = "bindings-wasm")]
pub mod bindings_wasm;

// #[cfg(feature = "bindings-wasm")]
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use halo2_base::{
    gates::{
        circuit::builder::BaseCircuitBuilder,
        flex_gate::{GateChip, GateInstructions},
    },
    halo2_proofs::halo2curves::{bn256::Fr, ff::PrimeField},
    poseidon::hasher::{spec::OptimizedPoseidonSpec, PoseidonHasher},
};
pub use halo2_wasm::Halo2Wasm;

#[derive(Clone, Debug)]
pub struct MyCircuit {
    // Add whatever other chips you need here
    gate: GateChip<Fr>,
    builder: Rc<RefCell<BaseCircuitBuilder<Fr>>>,
}

impl MyCircuit {
    pub fn new(circuit: &Halo2Wasm) -> Self {
        let gate = GateChip::new();
        MyCircuit {
            gate,
            builder: Rc::clone(&circuit.circuit),
        }
    }

    pub fn run(&mut self, iterations: u32) -> String {
        // Replace with your circuit, making sure to use `self.builder`
        let input = (0..iterations as usize).map(|i| {
            self.builder
                .borrow_mut()
                .main(0)
                .load_witness(Fr::from_u128(i as u128))
        });
        let mut poseidon =
            PoseidonHasher::<Fr, 3, 2>::new(OptimizedPoseidonSpec::new::<
                56,
                8,
                0,
            >());
        poseidon
            .initialize_consts(self.builder.borrow_mut().main(0), &self.gate);

        let result = input
            .reduce(|acc, a| {
                poseidon.hash_fix_len_array(
                    self.builder.borrow_mut().main(0),
                    &self.gate,
                    &[acc, a],
                )
            })
            .unwrap();

        let result_as_string = format!("{:?}", result);
        result_as_string
    }
}

#[cfg(test)]
mod test {

    use halo2_wasm::Halo2Wasm;

    use super::*;

    #[test]
    fn test_circuit() {
        let wasm_circuit = Halo2Wasm::default();
        let mut my_circuit = MyCircuit::new(&wasm_circuit);
        my_circuit.run(10);
    }
}
