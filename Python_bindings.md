---
icon: plug
order: 5
---

Python bindings are built for `ezkl` using [PyO3](https://pyo3.rs) and [Maturin](https://github.com/PyO3/maturin). This is done so to allow users of `ezkl` to leverage on the rich Data Science ecosystem that Python has instead of using Rust only. You can find the `ezkl` package on Pypi [here](https://pypi.org/project/ezkl/).  

### Using the library
Install `ezkl` with pip:
```bash
pip install ezkl
```

The `ezkl` python library provides all the functions you need to make a SNARK from your model. The functions are:
- `export(circuit: torch.nn, input_shape: [int])`: export your neural net to a .onnx input.json files
- `gen_srs(params_path, logrows: int <= 28)`: Generates the structured reference string for the circuit
- `setup(data_path, model_path, vk_path, pk_path, params_path, circuit_params_path)`: Sets up the circuit and generates circuit parameters and proving & verifying keys at the mentioned paths
- `prove(data_path, model_path, pk_path, proof_path, params_path, transcript-type (e.g. "poseidon"), strategy-type: (e.g. "single"), circuit_params_path)`: Generates a proof for a model
- `verify(proof_path, circuit_params_path, vk_path, params_path,)`: Verifies a proof

You can check out our Jupyter Notebook example [here](https://github.com/zkonduit/pyezkl/blob/main/examples/ezkl_demo.ipynb) for reference.
