use console_error_panic_hook;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MyCircuit(crate::MyCircuit);

#[wasm_bindgen]
impl MyCircuit {
    #[wasm_bindgen(constructor)]
    pub fn new(k: u32) -> Self {
        MyCircuit(crate::MyCircuit::new(k))
    }

    #[wasm_bindgen]
    pub fn prove(&self, a: &[u8], b: &[u8]) -> String {
        self.0.prove(a, b)
    }
}
