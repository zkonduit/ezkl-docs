---
icon: gear
order: 95
---

### Installing EZKL
You may want to download a release binary from the GitHub, or use `ezkl` from Python with `pip install ezkl`. If you want the latest build, you can also install from source. 

`ezkl` uses your system `solc` Solidity compiler, so you may need to tweak it using svm-rs or solc-select, particularly if you are targeting a specific hardfork.

### Building from source 🔨
Ezkl is built in rust. First [install rust](https://www.rust-lang.org/tools/install), e.g. by 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then download the repo and enter the directory
```bash
git clone git@github.com:zkonduit/ezkl.git
cd ezkl
```
We require a specific nightly version of the rust toolchain. You can change the default toolchain by running:
```bash
rustup override set nightly-2023-04-16
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
