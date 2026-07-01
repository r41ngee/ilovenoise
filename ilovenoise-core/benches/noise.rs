use criterion::{criterion_group, criterion_main, Criterion};
use ilovenoise_core::{
    algo::{self, Algorithm},
    image::Image,
    tasking::{self, TaskConfig},
    create_mode,
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
        let size = (task.width, task.height);
        group.bench_with_input(label, &(task, size, out), |b, &(t, s, ref out_path)| {
            b.iter(|| {
                let config = TaskConfig {
                    output: Some(out_path.clone()),
                    ..t.clone()
                };
                let rng = ChaCha8Rng::seed_from_u64(config.seed.unwrap_or(0));
                let mut algo = create_mode(rng, s, &config).unwrap();
                let mut image = Image::new(s);
                algo.draw(&mut image);
                save_bench_image(&image, config.output.as_deref().unwrap_or("output.png"));
            });
        });
    }
    group.finish();
}

fn save_bench_image(image: &Image, filepath: &str) {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new(filepath);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.size.0, image.size.1);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let image_data = image.as_bytes();
    writer.write_image_data(&image_data).unwrap();
    writer.finish().unwrap();
}

criterion_group!(benches, bench_random_noise, bench_perlin, bench_task_file);
criterion_main!(benches);
