use criterion::*;
use serde_json::from_slice;
use benchmark_ing::A;

fn fibonacci(i: i32) -> i32 {
    1
}

fn deserialize(i: &[u8]) -> A {
    let a = from_slice::<A>(i).unwrap();
    return a;
}

fn throughput_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput-group");
    // group.sample_size(1000000);
    let elements_1: &[u8] = b"{\"a\":\"a\"}";
    let elements_2: &[u8] = b"{\"c\": \"c\", \"a\":\"bbbbbbbbbbbbbbbbbbb\"}";
    for (i, elements) in [elements_1, elements_2].iter().enumerate() {
        group.throughput(Throughput::Bytes(elements.len() as u64));
        group.bench_with_input(format!("{}", i), elements, |b, &elems| {
            b.iter(||deserialize(elems))
        });
    }
    group.finish()
}

criterion_group!(benches, throughput_bench);
criterion_main!(benches);
