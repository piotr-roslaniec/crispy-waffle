extern crate wasm_bindgen_test;

use shielder_wasm::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn tdec_my_circuit() {
    let my_circuit = MyWasmCircuit::new(&Halo2Wasm::default());
    my_circuit.run(10);
}
