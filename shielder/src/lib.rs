#[cfg(feature = "wasm")]
extern crate alloc;

use shielder_circuits::{proof_generate, setup_params};

#[cfg(feature = "wasm")]
pub mod wasm;

#[derive(Clone, Debug)]
pub struct MyCircuit {
    setup_params: Vec<u8>,
}

impl MyCircuit {
    pub fn new(k: u32) -> Self {
        let setup_params = setup_params(k);
        MyCircuit { setup_params }
    }

    pub fn prove(&self, a: &[u8], b: &[u8]) -> String {
        let proof = proof_generate(a, b, &self.setup_params.to_vec());
        format!("{:?}", proof.to_vec())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_circuit() {}
}
