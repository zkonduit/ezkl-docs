---
icon: gear
order: 9
---

### building the project 🔨
Note that the library requires a nightly version of the rust toolchain. You can change the default toolchain by running:
```bash
rustup override set nightly
```

After which you may build and install the library
```bash
cargo install --force --path .
```

If you want to build manually with cargo build, be sure to use the release flag as the debug build will result in slow proofs
```bash
cargo build --release --bin ezkl
```
---------
##### Rust docs 📖

Use `cargo doc --open` to compile and open the Rust documentation for `ezkl` in your default browser.

### Things to consider
This section is meant to give our users some warnings and precautions about using `ezkl`. 
##### Quantization
In order to create a SNARK of a neural network, we must quantize the model parameters. In ML, parameters are almost always floating point numbers. In ezkl, we transform these to field elements so that we can use the zero knowledge proving system appropriately. Though we preserve as much precision as possible with our `--scale` flag (discussed more under the `Commands` section), outputs can still have some margin of error that should be accounted for. 