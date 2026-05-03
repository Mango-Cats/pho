# 🍜 / pho

**Pho**netic similarity algorithms.

> The 🍜 is still *cooking*. Everything is subject to change (even
> the visibility of the repository).

The name: pho is inspired by my love of Vietnamese *food*.

## How To

pho is written in Rust so be sure to have Rust installed.

This isn't available in Cargo. So, to use this on your own project
simply add this in the `[dependencies]` of your project's `Cargo.toml`:

```toml
pho = { git = "https://github.com/Mango-Cats/pho"}
```

Then run

```bash
cargo build
```

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
