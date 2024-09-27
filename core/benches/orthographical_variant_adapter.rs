use criterion::measurement::WallTime;
use criterion::{BatchSize, BenchmarkGroup, BenchmarkId, Criterion};
use japanese_address_parser::parser::adapter::orthographical_variant_adapter::{
    OrthographicalVariantAdapter, OrthographicalVariants, Variant,
};

pub fn bench_orthographical_variant_adapter(c: &mut Criterion) {
    let mut group = c.benchmark_group("orthographical_variant_adapter");
    add_tests(
        &mut group,
        TestSuite {
            expected: "松ケ崎東池ノ内町",
            inputs: vec![
                "松が崎東池ノ内町",
                "松ヶ崎東池ノ内町",
                "松ケ﨑東池ノ内町",
                "松ケ﨑東池の内町",
                "松ガ﨑東池の内町",
            ],
            variants_to_be_used: vec![Variant::ケ, Variant::崎, Variant::の],
        },
    );
    group.finish();
}

fn add_tests(group: &mut BenchmarkGroup<WallTime>, test_suite: TestSuite) {
    for input in test_suite.inputs {
        let benchmark_id = BenchmarkId::new(test_suite.expected, input);
        group.bench_with_input(benchmark_id, input, |b, input| {
            b.iter_batched(
                || OrthographicalVariantAdapter {
                    variant_list: test_suite.variants_to_be_used.clone(),
                },
                |adapter| {
                    let (region_name, _) = adapter.apply(input, test_suite.expected).unwrap();
                    assert_eq!(region_name, test_suite.expected);
                },
                BatchSize::SmallInput,
            )
        });
    }
}

struct TestSuite {
    expected: &'static str,
    inputs: Vec<&'static str>,
    variants_to_be_used: Vec<Variant>,
}
