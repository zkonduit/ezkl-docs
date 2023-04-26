---
icon: book
order: 1
---
## Examples 📖

## onnx examples

This repository includes onnx example files as a submodule for testing out the cli.

If you want to add a model to `examples/onnx`, open a PR creating a new folder within `examples/onnx` with a descriptive model name. This folder should contain:
- an `input.json` input file, with the fields expected by the  [ezkl](https://github.com/zkonduit/ezkl) cli.
- a `network.onnx` file representing the trained model
- a `gen.py` file for generating the `.json` and `.onnx` files following the general structure of `examples/tutorial/tutorial.py`.


TODO: add associated python files in the onnx model directories.

----------------------


## library examples 🔍

Beyond the `.onnx` examples detailed above, we also include examples which directly use some of our rust API; allowing users to code up computational graphs and circuits from scratch in rust without having to go via python.

The MNIST inference example using ezkl as a library is contained in `examples/conv2d_mnist`. To run it:

```bash
# download MNIST data
chmod +x data.sh
./data.sh
# test the model (takes 600-700 seconds)
cargo run --release --example conv2d_mnist
```

We also provide an example which runs an MLP on input data with four dimensions. To run it:

```bash
cargo run --release --example mlp_4d
```

----------------------


## Compiling to wasm 💾

The cli can also be compiled to for `wasm32-wasi` target (browser bindings with `wasm32-unknown-unknown` coming soon). To do so first ensure that [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) is installed.


You can then run:
```bash
rustup target add wasm32-wasi

wasm-pack build --bin ezkl --target wasm32-wasi
```
>Note: On Mac you may need to install llvm and clang using homebrew then explicitly set the `CC` and `AR` environment variables. For instance: `AR=/opt/homebrew/opt/llvm/bin/llvm-ar CC=/opt/homebrew/opt/llvm/bin/clang wasm-pack build --bin ezkl --target wasm32-wasi`

You can then run the compiled `.wasm` file as you would the normal cli detailed above (just not the `EVM` related commands), by using [wasmtime](https://docs.wasmtime.dev/cli-install.html).

```bash
wasmtime './target/wasm32-wasi/release/ezkl.wasm' -- --help
```
----------------------

## python bindings
Python bindings are built for `ezkl` using [PyO3](https://pyo3.rs) and [Maturin](https://github.com/PyO3/maturin). This is done so to allow users of `ezkl` to leverage on the rich Data Science ecosystem that Python has instead of using Rust only.

### production
Production Python bindings are made available via [pyezkl](https://github.com/zkonduit/pyezkl).


### development
To test the developmental Python bindings you will need to install [Python3](https://realpython.com/installing-python/). `ezkl` only supports version of python where `python >=3.7`.

Once python is installed setup a virtual environment and install `maturin`
```bash
python -m venv .env
source .env/bin/activate
pip install -r requirements.txt
```

You can now build the package for development and enable python bindings.
```bash
maturin develop --features python-bindings
```

Once done you will be able to access `ezkl_lib` as a python import as follows.
```python
import ezkl_lib
```

You may test if the existing build is working properly.
```
pytest
```

The list of python functions that can be accessed are found within `src/python.rs`