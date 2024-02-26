use criterion::criterion_main;
mod benchmarks;

criterion_main! {
    benchmarks::my_circuit::my_circuit,
}
