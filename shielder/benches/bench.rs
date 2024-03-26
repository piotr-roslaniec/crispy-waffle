use criterion::criterion_main;
mod circuits;

criterion_main! {
    circuits::hammster::hammster,
    circuits::merkle::merkle,
}
