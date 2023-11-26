use criterion::{black_box, criterion_group, criterion_main, Criterion};
use de_cart_contract::{self, marketplace};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "customer", 
        |b| b.iter(|| marketplace::register_customer(black_box("2vxsx-fae".into())))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);