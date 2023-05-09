---
icon: book
order: 6
---

This repository includes onnx example files as a submodule for testing out the cli.

If you want to add a model to `examples/onnx`, open a PR creating a new folder within `examples/onnx` with a descriptive model name. This folder should contain:
- an `input.json` input file, with the fields expected by the  [ezkl](https://github.com/zkonduit/ezkl) cli.
- a `network.onnx` file representing the trained model
- a `gen.py` file for generating the `.json` and `.onnx` files following the general structure of `examples/tutorial/tutorial.py`.


TODO: add associated python files in the onnx model directories.

----------------------


## library examples 🔍

Beyond the `.onnx` examples detailed above, we also include examples which directly use some of our rust API; allowing users to code up computational graphs and circuits from scratch in rust without having to go via python.

The MNIST inference example using `ezkl` as a library is contained in `examples/conv2d_mnist`. To run it:

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




