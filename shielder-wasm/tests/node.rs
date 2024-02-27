extern crate wasm_bindgen_test;

use shielder_wasm::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn tdec_my_circuit() {
    let my_circuit = MyCircuit::new(1);
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    let proof = my_circuit.prove(&a, &b);
    assert!(!proof.is_empty());
}
