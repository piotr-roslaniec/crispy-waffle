#![feature(test)]
extern crate test;

use test::Bencher;
use halo2_wasm::Halo2Wasm;
use halo2_wasm_template::MyCircuit;

#[bench]
fn bench_run(b: &mut Bencher) {
    let wasm_circuit = Halo2Wasm::default();
    let mut my_circuit = MyCircuit::new(&wasm_circuit);
    b.iter(|| my_circuit.run(10));
}