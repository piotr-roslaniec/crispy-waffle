use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use rand::Rng;
use shielder::MyCircuit;

pub fn bench_my_circuit(c: &mut Criterion) {
    let mut group = c.benchmark_group("MyCircuit");
    group.sample_size(10);

    // Input values to generate a proof with
    // TODO: Just one size for now, must match size used in `shield-runner-web`
    for size in [13].iter() {
        // Fill a and b with ones and zeros
        let mut rng = rand::thread_rng();
        let a: Vec<u8> = (0..*size).map(|_| rng.gen_range(0..2)).collect();
        let b: Vec<u8> = (0..*size).map(|_| rng.gen_range(0..2)).collect();

        // Size of the circuit. Circuit must fit within 2^k rows.
        let my_circuit = MyCircuit::new(*size);

        // Using a closure and cloning to avoid accidentally
        // benchmarking the setup code and mutating the circuit
        // multiple times
        // TODO: Not needed right now, but keeping this pattern for future reference
        let do_benchmark = {
            let my_circuit = my_circuit.clone();
            move || {
                black_box(my_circuit.prove(&a, &b));
            }
        };

        group.bench_function(BenchmarkId::new("run", size), |b| {
            b.iter(|| do_benchmark())
        });
    }
}

criterion_group!(merkle, bench_my_circuit);
