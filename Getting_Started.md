---
icon: gear
order: 9
---

### building the project 🔨
Ezkl is built in rust. First [install rust](https://www.rust-lang.org/tools/install), e.g. by 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then download the repo and enter the directory
```bash
git clone git@github.com:zkonduit/ezkl.git
cd ezkl
```
We require a nightly version of the rust toolchain. You can change the default toolchain by running:
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
> Note: To render your model circuits, you'll need to compile `ezkl` with the `render` feature (`cargo build --features render --bin ezkl`). This enables the `render-circuit` command which can create `.png` representations of the compiled circuits. You'll also need to install the `libexpat1-dev` and `libfreetype6-dev` libraries on Debian systems (there are equivalents for MacOS as well).
---------
##### Rust docs 📖

Use `cargo doc --open` to compile and open the Rust documentation for `ezkl` in your default browser.

### Things to consider
This section is meant to give our users some warnings and precautions about using `ezkl`. 
##### Quantization
In order to create a SNARK of a neural network, we must quantize the model parameters. In ML, parameters are almost always floating point numbers. In ezkl, we transform these to field elements so that we can use the zero knowledge proving system appropriately. Though we preserve as much precision as possible with our `--scale` flag (discussed more under the `Commands` section), outputs can still have some margin of error that should be accounted for. 
