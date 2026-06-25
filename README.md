# ilovenoise

CLI tool to generate PNG images with procedural noise. Supports Perlin noise with configurable fractal Brownian motion (fBM).

## Usage

```
ilovenoise [-w 512] [-h 512] [-s <seed>] [-o output.png] [-d]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-w` | Image width (must be multiple of 8) | `256` |
| `-h` | Image height (must be multiple of 8) | `256` |
| `-s` | Random seed (omit for random) | random |
| `-o` | Output path | `output.png` |
| `-d` | Automatical default settings usage |

After launching, an interactive menu asks for:
- **Algorithm**: `Random` (white noise) or `Perlin`
- **Perlin parameters** (if selected):
  - `Octaves` — number of fBM octaves (default: `4`)
  - `Persistence` — amplitude multiplier per octave (default: `0.5`)
  - `Lacunarity` — frequency multiplier per octave (default: `2.0`)

Press Enter or use `-d` flag to accept defaults.

## Example

```sh
ilovenoise -w 512 -h 512
# Select "Perlin"
# Press Enter for all defaults
# → output.png with 4-octave Perlin noise
```

## Dependencies

- `clap` — CLI argument parsing
- `dialoguer` — interactive prompts
- `png` — PNG encoding
- `rand` / `rand_chacha` — deterministic RNG
