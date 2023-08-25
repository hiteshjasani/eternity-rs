use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eternity_rs::Eternity;


fn bench_human(seconds: u64) -> String {
  Duration::from_secs(seconds).humanize()
}

fn bench_bot(seconds: u64) -> String {
  Duration::from_secs(seconds).robotize()
}


fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("human", |b| b.iter(|| bench_human(black_box((24 * 3600) + 3672))));
  c.bench_function("robot", |b| b.iter(|| bench_bot(black_box((24 * 3600) + 3672))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
