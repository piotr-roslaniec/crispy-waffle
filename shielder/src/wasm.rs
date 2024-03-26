use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn copy_vec_to_u8arr(v: &[u8]) -> Uint8Array {
    let u8_arr = Uint8Array::new_with_length(v.len() as u32);
    u8_arr.copy_from(v);
    u8_arr
}

#[wasm_bindgen]
pub fn setup_params(k: u32) -> Uint8Array {
    log("running setup");
    let buf = crate::setup_params(k);
    copy_vec_to_u8arr(&buf)
}

#[wasm_bindgen]
pub fn proof_generate(a: &[u8], b: &[u8], params_bytes: &[u8]) -> Uint8Array {
    log("proving...");
    let proof = crate::proof_generate(a, b, params_bytes);
    copy_vec_to_u8arr(&proof)
}

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
