use ascii::app::convert::convert_to_ascii;
use criterion::{criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
fn bench_pug(c: &mut Criterion) {
    c.bench_function("ascii pug", |b| {
        b.iter(|| convert_to_ascii(image::open("/home/maixnor/Pictures/pug.png").unwrap()))
    });
}

criterion_group!(benches, bench_pug);
criterion_main!(benches);
