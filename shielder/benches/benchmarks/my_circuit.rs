use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use shielder::{Halo2Wasm, MyCircuit};

pub fn bench_my_circuit(c: &mut Criterion) {
    let mut group = c.benchmark_group("MyCircuit");
    group.sample_size(10);

    let my_circuit = MyCircuit::new(&Halo2Wasm::default());

    for iterations in [10, 100, 200, 300].iter() {
        // Using a closure and cloning to avoid accidentally
        // benchmarking the setup code and mutating the circuit
        // multiple times
        let mut do_benchmark = {
            let mut my_circuit = my_circuit.clone();
            move || {
                black_box(my_circuit.run(*iterations));
            }
        };

        group.bench_function(BenchmarkId::new("run", iterations), |b| {
            b.iter(|| do_benchmark())
        });
    }
}

criterion_group!(my_circuit, bench_my_circuit);
