use criterion::{black_box, criterion_group, criterion_main, Criterion};
use de_cart_contract::{self, marketplace};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "merchant", 
        |b| b.iter(|| marketplace::register_merchant(black_box("2vxsx-fae".into())))
    );
}


/*
 * 
 * Benchmarking various data-structures for stroring 
 * encrypted data
 * 
 * Vec, VecDeque, LinkedList
 * HashMap, BTreeMap
 * HashSet BTreeSet
 * 
 * 1 record
 * 100 records
 * 10k records
 * 100k records
 * 750k records
 * 
 * Choose best structure and revise design accordingly
 */

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);