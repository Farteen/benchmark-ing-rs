use benchmark_ing::A;
use criterion::*;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

fn serde_json_deserialize(i: &[u8]) -> Value {
    let a = serde_json::from_slice(i).unwrap();
    return a;
}

fn throughput_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde-json-throughput-group");

    // group.sample_size(1000000);
    let mut file_byte = &mut Vec::new();
    let file_path = PathBuf::from_str("xx/data/canada.json").unwrap();
    File::open(file_path)
        .unwrap()
        .read_to_end(file_byte)
        .unwrap();
    group.throughput(Throughput::Bytes(file_byte.len() as u64));
    group.bench_function("serde_json", |b| {
        b.iter(|| serde_json_deserialize(black_box(file_byte)))
    });
    group.finish()
}

criterion_group!(benches, throughput_bench);
criterion_main!(benches);
