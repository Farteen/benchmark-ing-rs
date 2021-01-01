use benchmark_ing::A;
use criterion::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::env;
use simd_json::value::owned::Value;

fn simd_json_deserialize(i: &mut [u8]) -> Value {
    simd_json::to_owned_value(i).unwrap()
}

fn throughput_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde-json-throughput-group");

    // group.sample_size(1000000);
    let mut file_byte = &mut Vec::new();
    let mut file_path = env::current_dir().unwrap();
    file_path.push(PathBuf::from_str("./data/canada.json").unwrap());
    println!("{:?}", file_path.clone());
    File::open(file_path)
        .unwrap()
        .read_to_end(file_byte)
        .unwrap();
    group.throughput(Throughput::Bytes(file_byte.len() as u64));
    group.bench_function("simd_json", |b| {
        b.iter(|| simd_json_deserialize(black_box(file_byte)))
    });
    group.finish()
}

criterion_group!(benches, throughput_bench);
criterion_main!(benches);
