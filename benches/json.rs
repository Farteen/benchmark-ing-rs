use criterion::*;
use benchmark_ing::A;

fn serde_json_deserialize(i: &[u8]) -> A {
    let a = serde_json::from_slice::<A>(i).unwrap();
    return a;
}

fn simd_json_deserialize(i: &mut [u8]) -> A {
    let a = simd_json::from_slice(i).unwrap();
    return a;
}

fn throughput_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput-group");

    // group.sample_size(1000000);
    let elements_1: &[u8] = b"{\"c\": \"c\", \"a\":\"bbbbbbbbbbbbbbbbbbb\"}";
    let mut elements_2: String = "{\"c\": \"c\", \"a\":\"bbbbbbbbbbbbbbbbbbb\"}".to_owned();
    group.throughput(Throughput::Bytes(elements_1.len() as u64));
    let mut mut_element_2 = unsafe{elements_2.as_bytes_mut()};
    group.bench_function("serde_json", |b| b.iter(||serde_json_deserialize(elements_1)));
    group.bench_function("simd_json", |b| b.iter(||simd_json_deserialize(mut_element_2)));
    group.finish()
}

criterion_group!(benches, throughput_bench);
criterion_main!(benches);
