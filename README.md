# ilovenoise

CLI tool to generate PNG images with procedural noise. Supports Perlin noise with configurable fractal Brownian motion (fBM).

## Usage

```
ilovenoise --algo <mode> [options]
ilovenoise --task-file <path>
ilovenoise --completions <shell>
```

| Flag | Description | Default |
|------|-------------|---------|
| `-w` / `--width` | Image width (multiple of 8) | `256` |
| `-h` / `--height` | Image height (multiple of 8) | `256` |
| `-s` / `--seed` | Random seed for reproducibility | random |
| `-o` / `--output` | Output PNG path | `output.png` |
| `-a` / `--algo` | Noise algorithm: `random` or `perlin` | — (required) |
| `-t` / `--task-file` | TOML file with batch tasks | — |
| `--octaves` | Number of fBM octaves (Perlin only) | `4` |
| `--persistence` | Amplitude multiplier per octave (Perlin only) | `0.5` |
| `--lacunarity` | Frequency multiplier per octave (Perlin only) | `2.0` |
| `--completions` | Generate shell completions (`bash`/`zsh`/`fish`/`powershell`/`elvish`) | — |

## Examples

```sh
# Perlin noise with custom params
ilovenoise --algo perlin -w 512 -h 512 --octaves 6 --persistence 0.3

# Random noise with a seed
ilovenoise --algo random -s 42 -o random.png

# Batch from TOML file
ilovenoise --task-file tasks.toml

# Generate shell completions
ilovenoise --completions zsh > ~/.zsh/completions/_ilovenoise
```

## Shell Completions

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`.

## TOML Task File (Batch Mode)

Tasks run in parallel via `rayon`. Each task is independent — own seed, size, output.

```toml
[[task]]
mode = "Perlin"
width = 256
height = 256
output = "perlin_256.png"
seed = 42

[task.perlin]
octaves = 4
persistence = 0.5
lacunarity = 2.0

[[task]]
mode = "Perlin"
width = 512
height = 512
output = "perlin_fbm.png"
seed = 123

[task.perlin]
octaves = 8
persistence = 0.3
lacunarity = 2.5

[[task]]
mode = "Random"
width = 256
height = 256
output = "random.png"
seed = 77

[[task]]
mode = "Perlin"
width = 128
height = 128
output = "perlin_defaults.png"

[[task]]
mode = "Random"
width = 64
height = 64
output = "tiny_random.png"
```

## Dependencies

- `clap` / `clap_complete` — CLI argument parsing and completions
- `png` — PNG encoding
- `rand` / `rand_chacha` — deterministic RNG
- `rayon` — multiprocessing
- `anyhow` — error handling
- `serde` / `toml` — task file deserialization
