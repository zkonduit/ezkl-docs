---
icon: gear
order: 95
---

### Installing EZKL

To use `ezkl` in Python, just `pip install ezkl`. You will generally also need `onnx` installed if you are exporting models, and Pytorch, Tensorflow, or similar if you are creating models.

`ezkl` uses your system `solc` Solidity compiler, so you may need to tweak it using svm-rs or solc-select, particularly if you are targeting a specific hardfork.

To use the cli, download a [release binary](https://github.com/zkonduit/ezkl/releases) from GitHub. If you want the latest build, you can also install from source.

### Building from source 🔨

Ezkl is built in rust. First [install rust](https://www.rust-lang.org/tools/install), then download the repo and enter the directory

```bash
git clone git@github.com:zkonduit/ezkl.git
cd ezkl
```

After which you may build and install the library

```bash
cargo install --force --path .
```

If you want to build manually with cargo build, be sure to use the release flag as the debug build will result in slow proofs

```bash
cargo build --release --bin ezkl
```

You can always check the options available for a command by typing the command with `--help`. For example, `ezkl table` will show you the options available for the `table` command. This will provide you with the most up-to-date information on a given command's usage and the cli spec.

```bash
# list all available commands
ezkl --help
```

> Note: To render your model circuits, you'll need to compile `ezkl` with the `render` feature (`cargo build --features render --bin ezkl`). This enables the `render-circuit` command which can create `.png` representations of the compiled circuits. You'll also need to install the `libexpat1-dev` and `libfreetype6-dev` libraries on Debian systems (there are equivalents for MacOS as well).
---------

##### Rust docs 📖

Use `cargo doc --open` to compile and open the Rust documentation for `ezkl` in your default browser.
