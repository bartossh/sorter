use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::{self, File};
use std::hint::black_box;
use std::io::Write;
use std::process::Command;
use std::time::Duration;

fn create_test_data(size: usize) -> String {
    let dir = "bench_data";
    fs::create_dir_all(dir).unwrap();

    let filename = format!("{}/bench_{}.txt", dir, size);
    let mut file = File::create(&filename).unwrap();

    let mut rng = 42u64;
    for _ in 0..size {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        writeln!(file, "{}", rng % 1_000_000_000).unwrap();
    }

    filename
}

fn run_sort(input: &str, output: &str, batch_size: usize) {
    Command::new("target/release/sort_bigger_then_ram")
        .args(["-i", input, "-o", output, "-b", &batch_size.to_string()])
        .output()
        .expect("Failed to run sort");
}

fn benchmark_small_file(c: &mut Criterion) {
    let input = create_test_data(1000);
    let output = "bench_data/output_small.txt";

    c.bench_function("sort_1k_numbers", |b| {
        b.iter(|| {
            run_sort(black_box(&input), black_box(output), black_box(100));
        });
    });

    let _ = fs::remove_file(output);
}

fn benchmark_medium_file(c: &mut Criterion) {
    let input = create_test_data(100_000);
    let output = "bench_data/output_medium.txt";

    let mut group = c.benchmark_group("medium_file");
    group.measurement_time(Duration::from_secs(30));

    group.bench_function("sort_100k_numbers_small_batch", |b| {
        b.iter(|| {
            run_sort(black_box(&input), black_box(output), black_box(1000));
        });
    });

    group.bench_function("sort_100k_numbers_large_batch", |b| {
        b.iter(|| {
            run_sort(black_box(&input), black_box(output), black_box(10000));
        });
    });

    group.finish();

    let _ = fs::remove_file(output);
}

fn benchmark_large_file(c: &mut Criterion) {
    let input = create_test_data(1_000_000);
    let output = "bench_data/output_large.txt";

    let mut group = c.benchmark_group("large_file");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);

    group.bench_function("sort_1m_numbers", |b| {
        b.iter(|| {
            run_sort(black_box(&input), black_box(output), black_box(10000));
        });
    });

    group.finish();

    let _ = fs::remove_file(output);
}

fn benchmark_batch_sizes(c: &mut Criterion) {
    let input = create_test_data(50_000);
    let output = "bench_data/output_batch.txt";

    let mut group = c.benchmark_group("batch_sizes");
    group.measurement_time(Duration::from_secs(20));

    for batch_size in &[100, 500, 1000, 5000, 10000] {
        group.bench_with_input(
            format!("batch_{}", batch_size),
            batch_size,
            |b, &batch_size| {
                b.iter(|| {
                    run_sort(black_box(&input), black_box(output), black_box(batch_size));
                });
            },
        );
    }

    group.finish();

    let _ = fs::remove_file(output);
}

criterion_group!(
    benches,
    benchmark_small_file,
    benchmark_medium_file,
    benchmark_large_file,
    benchmark_batch_sizes
);
criterion_main!(benches);
