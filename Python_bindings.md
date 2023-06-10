---
icon: plug
order: 5
---

### using ezkl from Python

```bash
pip install ezkl
```
lets you use `ezkl` directly from Python. It also contains an `export` function to generate `.onnx` and `.json` input files that can be ingested by the `ezkl` cli or from Python. [Here is colab notebook](https://colab.research.google.com/drive/1XuXNKqH7axOelZXyU3gpoTOCvFetIsKu?usp=sharing) that shows how to produce and verify a proof from Python.

These Python bindings are developed in [pyezkl](https://github.com/zkonduit/pyezkl).


### development
Python bindings are built for `ezkl` using [PyO3](https://pyo3.rs) and [Maturin](https://github.com/PyO3/maturin). 

To test the development Python bindings you will need to install [Python3](https://realpython.com/installing-python/). `ezkl` only supports versions of python where `python >=3.7`.

Once python is installed setup a virtual environment and install `maturin`
```bash
python -m venv .env
source .env/bin/activate
pip install -r requirements.txt
```

You can now build the package for development and enable python bindings. The following command will install `ezkl_lib` into your local python environment.
```bash
# Unoptimized development build, use this if you require visibility regarding Rust errors
# Note that this can result in long proving/verification times, if this is a problem use the optimized development build below
maturin develop --features python-bindings

# Optimized development build, use this if you find that the proving/verification times become long
maturin develop --release --features python-bindings
```

Once done you will be able to access `ezkl_lib` as a python import as follows.
```python
import ezkl_lib
```

You may test if the existing build is working properly.
```bash
pytest
```

The list of python functions that can be accessed are found within `src/python.rs`
