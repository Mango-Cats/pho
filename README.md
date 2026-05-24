# 🍜 / pho

(**Pho**netic) similarity algorithms.

> The 🍜 is still *cooking*. Everything is subject to change (even
> the visibility of the repository).

The name: pho is inspired by my love of Vietnamese *food*.

## How To

pho is written in [Rust](https://rust-lang.org/tools/install/).

This isn't available in Cargo. So, to use this on your own project
simply add this in the `[dependencies]` of your project's `Cargo.toml`:

```toml
pho = { git = "https://github.com/Mango-Cats/pho"}
```

Then run

```bash
cargo build
```

**Examples**: To get a feel of what this project does, you can look
at the [examples/](examples/) folder.

**Jupyter**: Experimenting with pho is better in a notebook
environment! Look at [evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md)
to get a Rust Jupyter Kernel.

## CLI with `phoc`

`phoc` generates similarity feature datasets from CSV input.

### Quick start

```bash
cargo run --bin phoc -- \
   --input examples/data/dataset.csv \
   --output output.csv \
   --progress
```

### Build only phoc

```bash
cargo build --bin phoc
```

### Configs

`phoc` loads algorithm configs from the directory you provide:

```bash
cargo run --bin phoc -- \
   --input examples/data/dataset.csv \
   --output output.csv \
   --config-dir path/to/configs
```

Sample configs are in `tests/config_sample_*.toml`. Copy those into a directory
of your choice and point `--config-dir` at it.

### Input CSV format

Headers are flexible. The following names are recognized:

- `x_1` or `x` for the first string
- `x_2` or `y` for the second string
- `label` (optional numeric label)
- `t_1` and `t_2` (optional phonetic transcriptions)

If a configured algorithm requires phonetic transcriptions (for example ALINE),
`t_1` and `t_2` must be present for all rows.

### Output CSV

The output CSV includes:

- `x_1`, `x_2`, `label`
- One column per algorithm (plus optional word-level features)

Example header:

```
x_1,x_2,label,Levenshtein,JaroWinkler,Prefix
```

### Options

- `--delimiter <char>`: CSV delimiter byte (default: `,`)
- `--no-headers`: treat input as headerless
- `--flexible`: allow variable-length rows
- `--include-word-features`: add length-based features
- `--progress`: show a progress bar

## Moving Around

The project has three main modules:

1. [`pho::algorithms`](src/algorithms/): source code for basis
   functions (e.g., Aline, Editex, Levenshtein).
2. [`pho::learning`](src/learning/): source code for learning weights
   used in an ensemble algorithm.
3. [`pho::ensemble`](src/ensemble/): source code for grouping basis
   functions to form an ensemble algorithm.

---

<img width="640" height="427" alt="kirill-tonkikh-NFQi_2HUNRI-unsplash" src="https://github.com/user-attachments/assets/96905b8e-1520-4bbb-92a0-9a76a5c66156" />
Photo by <a href="https://unsplash.com/@photophotostock?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Kirill Tonkikh</a> on <a href="https://unsplash.com/photos/a-bowl-of-noodle-soup-with-chopsticks-on-the-side-NFQi_2HUNRI?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText">Unsplash</a>.
