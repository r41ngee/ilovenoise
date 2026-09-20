# ilovenoise-core

Procedural noise generation algorithms. Part of the [ilovenoise](https://crates.io/crates/ilovenoise) project.

## Usage

```toml
[dependencies]
ilovenoise-core = "0.1"
```

```rust
use ilovenoise_core::{algo::perlin::Perlin, image::Image};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

let size = (256, 256);
let rng = ChaCha8Rng::seed_from_u64(42);
let mut perlin = Perlin::new(size, rng, Some(4), Some(0.5), Some(2.0));
let mut image = Image::new(size);
perlin.draw(&mut image);
```

## Features

- `tasking` (default) — `TaskConfig`, `PerlinConfig`, `load_tasks` for batch configuration via TOML
- Without `tasking` — only `algo`, `image`, `util` types (no serde/toml deps)

## Algorithms

| Module | Description |
|--------|-------------|
| `algo::random_noise::RandomNoise` | Per-pixel white noise |
| `algo::perlin::Perlin` | Perlin noise with configurable fBM (octaves, persistence, lacunarity) |
