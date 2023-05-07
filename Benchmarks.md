---
icon: thumbsup
order: 4
---

We include proof generation time benchmarks for some of the implemented layers including the affine, convolutional, and ReLu operations (more to come).

To run these benchmarks:

```bash
cargo bench
```

To run a specific benchmark append one of `affine, cnvrl, relu` to the command. You can then find benchmarks results and plots in `target/criterion`. Note that depending on the capabilities of your machine you may need to increase the target time on the Criterion config. For instance:

```rust
criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(10));
  targets = runrelu
}
```
----------------------