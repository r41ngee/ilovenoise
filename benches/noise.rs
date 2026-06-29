use criterion::{criterion_group, criterion_main, Criterion};
use ilovenoise::{
    algo::{self, Aglorithm},
    image::Image,
    tasking::{self, TaskConfig},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

// ── Pure generation benchmarks ──────────────────────────────

fn bench_random_noise(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_noise/generation");

    for &size in &[(64, 64), (256, 256), (1024, 1024)] {
        group.bench_with_input(format!("{}x{}", size.0, size.1), &size, |b, &s| {
            let rng = ChaCha8Rng::seed_from_u64(42);
            let mut algo = algo::random_noise::RandomNoise::new(rng);
            let mut image = Image::new(s);
            b.iter(|| algo.draw(std::hint::black_box(&mut image)));
        });
    }
    group.finish();
}

fn bench_perlin(c: &mut Criterion) {
    let mut group = c.benchmark_group("perlin/generation");

    let configs: &[(&str, (u32, u32), u32, f32, f64)] = &[
        ("64x64_oct4", (64, 64), 4, 0.5, 2.0),
        ("256x256_oct4", (256, 256), 4, 0.5, 2.0),
        ("256x256_oct8", (256, 256), 8, 0.5, 2.0),
        ("1024x1024_oct4", (1024, 1024), 4, 0.5, 2.0),
    ];

    for &(name, size, octaves, persistence, lacunarity) in configs {
        group.bench_with_input(
            name,
            &(size, octaves, persistence, lacunarity),
            |b, &(s, oct, per, lac)| {
                let rng = ChaCha8Rng::seed_from_u64(42);
                let mut algo =
                    algo::perlin::Perlin::new(s, rng, Some(oct), Some(per), Some(lac));
                let mut image = Image::new(s);
                b.iter(|| algo.draw(std::hint::black_box(&mut image)));
            },
        );
    }
    group.finish();
}

// ── Full iteration from task file ───────────────────────────

fn bench_task_file(c: &mut Criterion) {
    let out = std::env::temp_dir().join("ilovenoise_bench.png");
    let out = out.to_str().unwrap().to_string();

    let mut group = c.benchmark_group("full_iteration");

    let tasks = tasking::load_tasks("benches/bench_tasks.toml").unwrap();
    for task in &tasks {
        let label = format!("{}_{}x{}", task.mode, task.width, task.height);
        let out = out.clone();
        group.bench_with_input(label, &(), |b, _| {
            b.iter(|| {
                let t = TaskConfig {
                    output: Some(out.clone()),
                    ..task.clone()
                };
                t.run().unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_random_noise, bench_perlin, bench_task_file);
criterion_main!(benches);
