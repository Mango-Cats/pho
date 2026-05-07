# examples @ 🍜 / pho

This folder contains examples (which are more or less tutorials) of
the ins and outs of pho.

To run an example:

```bash
cargo run --example <filename>
```

## Crash course

Visit the examples (in the order):

1. [Working with algorithm configurations](/example/configs.rs)
2. [Running algorithms](/example/algorithm.rs)
3. [Creating and running ensembles](/example/ensemble.rs)
4. [Working with datasets](/example/dataset.rs)
5. [ALINE datasets with transcriptions](/example/aline_dataset.rs)
6. Learning weights via Genetic Algorithms:
   1. [Introduction](/example/ga_intro.rs)
   1. [Case Study](/example/ga_extreme_case.rs)

## Running algorithms from previous literature

We also make available implementations of algorithms from previous
literature.

1. [Lambert (1999)](/examples/lambert.rs)
1. [Kondrak and Dorr (2004)](/examples/kondrak_dorr.rs)

## Others

Below is a list of miscellaneous examples and what they're for:

1. [Mass Precomputation](/examples/mass_precompute.rs). This example
   constructs a single dataset that contains precomputation of all the
   algorithms defined in pho.
