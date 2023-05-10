---
icon: plug
order: 5
---

Python bindings are built for `ezkl` using [PyO3](https://pyo3.rs) and [Maturin](https://github.com/PyO3/maturin). This is done so to allow users of `ezkl` to leverage on the rich Data Science ecosystem that Python has instead of using Rust only.

Check out our Jupyter Notebook example [here](https://github.com/zkonduit/ezkl/blob/main/examples/notebook/ezkl_demo.ipynb)

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
