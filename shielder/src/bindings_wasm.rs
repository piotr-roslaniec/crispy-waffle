use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Halo2Wasm, MyCircuit};

#[wasm_bindgen]
pub struct MyWasmCircuit(MyCircuit);

#[wasm_bindgen]
impl MyWasmCircuit {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let circuit = Halo2Wasm::default();
        MyWasmCircuit(MyCircuit::new(&circuit))
    }

    #[wasm_bindgen]
    pub fn run(&mut self, iterations: u32) -> String {
        self.0.run(iterations)
    }
}
