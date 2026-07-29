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

Every `.toml` file in that directory produces **one output feature column**:

- The **column name is the file name** (its stem). `my_sim.toml` produces a
  `my_sim` column, so you can run the same algorithm twice under different names.
- The **algorithm is chosen by the file's `algorithm` key**, not the file name.
  For example, a file containing `algorithm = "levenshtein"` uses Levenshtein
  regardless of what the file is called.
- The rest of the file is that algorithm's own config.

```toml
# configs/my_sim.toml
algorithm = "levenshtein"
case_insensitive = false

[costs]
insert = 1.0
delete = 1.0
substitute = 1.0
```

There is **no implicit set of always-on algorithms**. Config-less algorithms
(like `lcs` and `lcsuf`) are included the same way — drop in a file whose only
required content is the `algorithm` key:

```toml
# configs/lcs.toml
algorithm = "lcs"
```

To exclude an algorithm, remove its file; to include one, add its file. Ready-made
configs live in [`algorithm_configs/eng`](algorithm_configs/eng). Recognized
`algorithm` values: `aline`, `bisim`, `double_metaphone`, `editex`, `jaro_winkler`,
`keyboard`, `lcs`, `lcsubstring`, `lcsuf`, `levenshtein`, `metaphone`,
`needleman_wunsch`, `ngram`, `prefix`, `smith_waterman`, `soundex`, `syllable`,
`tfidf`, `visual_weighted`.

Levenshtein also accepts a `consonants_only = true` key to strip vowels
(`a`, `e`, `i`, `o`, `u`; `y` stays a consonant) from both inputs before
scoring — see
[`algorithm_configs/eng/levenshtein_consonant.toml`](algorithm_configs/eng/levenshtein_consonant.toml).

#### Separated edit-operation counts

Levenshtein, Editex, and `visual_weighted` support a `separate = true` key. When
set, the config produces **three** columns instead of one — `{stem}_substitutions`,
`{stem}_insertions`, and `{stem}_deletions` — each the literal count of that
operation type in the minimal-cost alignment, rather than a single summed
distance:

```toml
# configs/lev_ops.toml
algorithm = "levenshtein"
separate = true

[costs]
insert = 1.0
delete = 1.0
substitute = 1.0
```

produces `lev_ops_substitutions,lev_ops_insertions,lev_ops_deletions` instead of
a single `lev_ops` column.

### Input CSV format

Headers are flexible. The following names are recognized:

- `x_1` or `x` for the first string
- `x_2` or `y` for the second string
- `label` (optional numeric label)
- `t_1` and `t_2` (optional phonetic transcriptions)

If a configured algorithm requires phonetic transcriptions (for example ALINE),
`t_1` and `t_2` must be present for all rows.

### Output CSV

The output preserves **every column of the input CSV verbatim** (including
`t_1`/`t_2` and any extra columns you keep for your own bookkeeping) and appends
one column per configured algorithm (plus optional word-level features). Only
`x_1`/`x_2`/`t_1`/`t_2` are used for scoring; all other input columns are ignored
for computation but carried through untouched.

Feature columns are named after their config files. Given `levenshtein.toml`,
`jaro_winkler.toml`, and `prefix.toml`, and an input with an extra `label` and
`source` column:

```
x_1,x_2,label,source,levenshtein,jaro_winkler,prefix
```

Row order matches the input.

### Options

- `--delimiter <char>`: CSV delimiter byte (default: `,`)
- `--no-headers`: treat input as headerless
- `--flexible`: allow variable-length rows
- `--include-word-features`: add structural/prosodic features computed on the
  raw `x_1`/`x_2` strings (no `case_insensitive` option): `len_x1`, `len_x2`,
  `len_min`, `len_max`, `len_diff`, `len_ratio`, `common_prefix_len`,
  `common_prefix_ratio`, `common_suffix_len`, `common_suffix_ratio`,
  `first_mismatch_pos`, `first_char_match`, `consonant_count_diff`,
  `syllable_diff`, `vowel_count_diff`. The last three are absolute-difference
  counts (consonant letters, vowel-nucleus runs, vowel letters) using the
  plain 5-vowel Latin set (`a`, `e`, `i`, `o`, `u`; no `y`). If a configured
  algorithm is named `Prefix`, its column is renamed to `common_prefix_ratio`
  instead of duplicating that feature.
- `--include-fil-features`: add Filipino (Tagalog) loanword-nativization
  indicators: `fil_vowel_skeleton_match`, `fil_penult_vowel_match`,
  `fil_onset_match`, `fil_coda_match`, `fil_phonetic_equal`. Each is `1`/`0`,
  not a similarity score — they flag whether a Filipino speaker would hear
  the pair as structurally alike *after* nativizing loanword spelling (e.g.
  `chocolate` and `tsokolate` nativize to the same spelling and agree on
  every indicator despite sharing no raw prefix). Nativization is done
  in-process via the [`tagabaybay`](https://github.com/Mango-Cats/tagabaybay)
  adapter with G2P-based adaptation disabled (it would otherwise shell out to
  a Python/espeak-ng subprocess on first use), so a handful of
  ambiguous-vowel words may nativize slightly differently here than through
  `tbb-cli`, which enables G2P by default.
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
