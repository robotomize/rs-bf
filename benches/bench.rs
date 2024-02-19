use criterion::{criterion_group, criterion_main, Criterion, black_box};
use bf::BloomFilter;
use uuid::Uuid;

fn bench_bloom_filter(c: &mut Criterion) {
    let mut bloom_filter = BloomFilter::new(20_000_000, 3);

    c.bench_function("add_to_bloom_filter", |b| {
        b.iter(|| {
            bloom_filter.add(Uuid::new_v4().to_string());
        });
        b.iter(|| {
            if let checked = bloom_filter.contains(Uuid::new_v4().to_string()) {
                black_box(checked);
            }
        });
    });
}

criterion_group!(benches, bench_bloom_filter);
criterion_main!(benches);
