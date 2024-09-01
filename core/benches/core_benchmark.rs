mod orthographical_variant_adapter;

use crate::orthographical_variant_adapter::bench_orthographical_variant_adapter;
use criterion::{criterion_group, criterion_main};

criterion_group!(benches, bench_orthographical_variant_adapter);
criterion_main!(benches);
