use criterion::{criterion_group, criterion_main, Criterion};
use de_cart_contract::customer::{Customer};
use std::collections::HashMap;

fn store(size: i32) -> HashMap<String, Customer> {
    let mut customers: HashMap<String, Customer> = HashMap::new();
    for x in 1..size {
        let mut new_customer = Customer::default();
        new_customer.id = x.to_string();
        customers.insert(x.to_string(), new_customer);
    }
    customers
}

fn benchmark_hashmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("insertion");

    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(120));

    // let customers = store(n);
    // group.throughput(Throughput::Elements(n as u64));
    // group.bench_with_input(BenchmarkId::new("Hashmap", n), &n, |b, &n|{
    //     b.iter(|| {
    //         insert(black_box(Customer::default()), black_box(customers.clone()))
    //     })
    // });

    let _store_10000 = store(10_000);
    let _store_100000 = store(100_000);
    let _store_1000000 = store(1_000_000);

    let vec_store: Vec<i32> = (1..=100).collect();

    // let store
    group.bench_function("vec_size_10_000", |b| {
        b.iter(|| vec_store.iter().find(|&x| *x == 99))
    });

    // group.bench_function("hashmap_size_100_000", |b| b.iter(|| insert(black_box(Customer::default()), black_box(store_100000.clone()))));
    // group.bench_function("hashmap_size_1_000_000", |b| b.iter(|| insert(black_box(Customer::default()), black_box(store_1000000.clone()))));

    group.finish()
}

fn insert(mut customer: Customer, mut store: HashMap<String, Customer>) {
    customer.id = "test".to_string();
    store.insert("test".to_string(), customer);
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

criterion_group!(benches, benchmark_hashmap);
criterion_main!(benches);
