use criterion::{criterion_group, criterion_main, Criterion};
use runtime_units::units::{CubeRootScaledLengthUnit, LengthUnit};
use runtime_units::{Length, CubeRootScaledLength};
use runtime_units::traits::FixedQuantity;
fn runtime_units_benchmarks(c: &mut Criterion) {
    c.bench_function("try_convert_bench_standard", |b| {
        b.iter(|| {
            // Simulate some work that would be done in a runtime unit
            let length = Length::meter(100.0);
            for _ in 0..1000000 {
                let _ = length.try_convert(LengthUnit::kilometer.into()).unwrap();
            }
        })
    });
}

fn runtime_units_benchmarks_rational(c: &mut Criterion) {
    c.bench_function("try_convert_bench_rational", |b| {
        b.iter(|| {
            // Simulate some work that would be done in a runtime unit
            let length = CubeRootScaledLength::scaled_meter_per_kiloton_tnt(100.0);
            for _ in 0..1000000 {
                let _ = length.try_convert(CubeRootScaledLengthUnit::scaled_foot_per_ton_tnt.into()).unwrap();
            }
        })
    });
}
fn runtime_units_benchmarks_same_type(c: &mut Criterion) {
    c.bench_function("try_convert_bench_standard_same_type", |b| {
        b.iter(|| {
            // Simulate some work that would be done in a runtime unit
            let length = Length::meter(100.0);
            for _ in 0..1000000 {
                let _ = length.convert(LengthUnit::kilometer.into());
            }
        })
    });
}

fn runtime_units_benchmarks_rational_same_type(c: &mut Criterion) {
    c.bench_function("try_convert_bench_rational_same_type", |b| {
        b.iter(|| {
            // Simulate some work that would be done in a runtime unit
            let length = CubeRootScaledLength::scaled_meter_per_kiloton_tnt(100.0);
            for _ in 0..1000000 {
                let _ = length.convert(CubeRootScaledLengthUnit::scaled_foot_per_ton_tnt);
            }
        })
    });
}
criterion_group!(try_convert_bench_standard, runtime_units_benchmarks, runtime_units_benchmarks_same_type);
criterion_group!(try_convert_bench_rational, runtime_units_benchmarks_rational, runtime_units_benchmarks_rational_same_type);
criterion_main!(try_convert_bench_rational, try_convert_bench_standard);