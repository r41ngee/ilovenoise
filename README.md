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
| `-s` / `--seed` | Random seed | random |
| `-o` / `--output` | Output path | `output.png` |
| `-a` / `--algo` | Algorithm: `random` or `perlin` | required |
| `-t` / `--task-file` | TOML task file for batch processing | — |
| `--octaves` | fBM octaves (Perlin only) | `4` |
| `--persistence` | Amplitude multiplier per octave | `0.5` |
| `--lacunarity` | Frequency multiplier per octave | `2.0` |
| `--completions` | Generate shell completions | — |

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

## TOML Task File

```toml
[[task]]
mode = "Perlin"
width = 512
height = 512
seed = 42

[task.perlin]
octaves = 6
persistence = 0.3
lacunarity = 2.5

[[task]]
mode = "Random"
width = 256
height = 256
output = "random.png"
```

## Dependencies

- `clap` / `clap_complete` — CLI argument parsing and completions
- `png` — PNG encoding
- `rand` / `rand_chacha` — deterministic RNG
- `rayon` — multiprocessing
- `serde` / `toml` — task file deserialization
