---
icon: plug
order: 5
---


### using ezkl from Python

```bash
pip install ezkl

python

>>> import ezkl
```

lets you use `ezkl` directly from Python.  [Here is a colab notebook](https://colab.research.google.com/drive/1XuXNKqH7axOelZXyU3gpoTOCvFetIsKu?usp=sharing) that shows how to produce and verify a proof from Python.

When installing `ezkl` with pip, you may want to use a virtualenv.
Some virtualenv management solutions for python includes `venv`, `pipenv`, `conda`, and `poetry`.

### development

Python bindings are built for `ezkl` using [PyO3](https://pyo3.rs) and [Maturin](https://github.com/PyO3/maturin).

To test the development Python bindings you will need to install [Python3](https://realpython.com/installing-python/).

Note, `ezkl` is only supported for `Python>=3.7`.

```bash
# using venv
python -m venv .env
source .env/bin/activate
pip install ezkl
```

#### 2. Install solc-select or svm-rs

To run solidity and evm related functionality make sure to have solc available in your environment.
We will need `solc >= 0.8.20`, otherwise contracts will fail to compile.
Otherwise, you are likely to encounter errors when dealing with solidity and evm related functionality that is used within ezkl.

It is recommended that you use [solc-select if you prefer a python based management solution](https://github.com/crytic/solc-select) or [svm if you prefer a rust based management solution](https://github.com/alloy-rs/svm-rs).
With a solidity version manager you are then able to change solidity versions in your environment easily.

#### 3. Try out EZKL Examples in the repository with a Jupyter Notebook

Clone the pyezkl repository.

```bash
git clone https://github.com/zkonduit/pyezkl.git
```

Install jupyter and start the jupyter notebook

```bash
pip install jupyter
jupyter notebook
```

Navigate to the [ezkl_demo.ipynb](https://github.com/zkonduit/pyezkl/blob/main/examples/ezkl_demo.ipynb) file which is located in the examples folder. It contains a minimal setup for running ezkl within python.

### Developmental python bindings

Setting up the development python bindings can be an involved process.

#### ezkl repository

In the event that you may want to use the developmental bindings on the main `ezkl` repository, you can clone and build the [main ezkl repository written in rust](https://github.com/zkonduit/ezkl) instead.

```bash
git clone https://github.com/zkonduit/ezkl.git
```

It's recommended that you set up a separate virtual environment for this.

```bash
# using venv
python -m venv .env
source .env/bn/activate
pip install -r requirements.txt
```

Ensure that rust is installed in your local environment, this is needed by maturin/pyo3 to build the project.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

Change the default toolchain to the nightly version as this is needed by some of the libraries used.

```bash
rustup override set nightly
```

After which, you should be able to build via `maturin build`. This will build `ezkl_lib` not `ezkl`.
`ezkl_lib` only includes the basic rust bindings without other Python functionality.

```bash
# Unoptimized development build, use this if you require visibility regarding Rust errors

# Note that this can result in long proving/verification times, if this is a problem use the optimized development build below

maturin build --features python-bindings


# Optimized development build, use this if you find that the proving/verification times become long

maturin develop --release --features python-bindings
```


### API Reference

![](../assets/library.png)
This reference details function, modules, and objects included in both `ezkl`.

You can find the full api reference [here](https://ezkl-ezkl.readthedocs-hosted.com).
