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

When installing `ezkl` with pip, you may want to use a virtualenv.
Some virtualenv management solutions for python includes `venv`, `pipenv`, `conda`, and `poetry`.


### development
Python bindings are built for `ezkl` using [PyO3](https://pyo3.rs) and [Maturin](https://github.com/PyO3/maturin). 

To test the development Python bindings you will need to install [Python3](https://realpython.com/installing-python/). `ezkl` only supports versions of python where `python >=3.7`.

Note, `ezkl` is only supported for `Python>=3.7`, this installs the [pyezkl build](https://github.com/zkonduit/pyezkl) which contains Python specific functions that the [Rust bindings on the main ezkl repository do not implement](https://github.com/zkonduit/ezkl).

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

maturin build --release --features python-bindings
```

Once the build is complete, you will be able to find the built wheels within `target/wheels`.
If your build is successful you should find a `.whl` file in the folder

Example:

```bash
ls target/wheels

ezkl_lib-0.1.0-cp37-abi3-manylinux_2_35_x86_64.whl

pip install ezkl_lib-0.1.0-cp37-abi3-manylinux_2_35_x86_64.whl
```

#### pyezkl repository

If you would like to then use the development version of [pyezkl](https://github.com/zkonduit/pyezkl) with the developmental bindings at [ezkl](https://github.com/zkonduit/ezkl), you will need to setup pyezkl too.


Clone the pyezkl repository in a separate directory if you haven't done so already.

```bash
git clone https://github.com/zkonduit/pyezkl.git
```

We will use `poetry` for the pyezkl repository. [Install poetry by following the instructions provided](https://python-poetry.org/docs/).

You will also need to deactivate any existing virtualenv.
```bash
deactivate
```

Once that is done setup the repository with `poetry`.

```bash
cd pyezkl
poetry install
poetry shell        # activates the virtual environment for poetry
```

Navigate to the ezkl repository and install the wheel that you have built.
```bash
cd ezkl/target/wheels
pip install ezkl_lib-version-pythontype-abi3-osversion-processor.whl
```

This will install the developmental `ezkl_lib` into the `poetry` environment.
After which, you should be able to build the developmental `ezkl` library from the `pyezkl` repository.

```bash
# navigate back to pyezkl
cd pyezkl
poetry build
pip install ./dist/ezkl-version-py3-none-any.whl
```

If successful, you should be able to run python in your `poetry` environment and call the functions.
```
python
>>> import ezkl
>>> ezkl.export(...)
```

### API Reference

This reference details function, modules, and objects included in both `ezkl` and `ezkl_lib`.
Note that `ezkl` is a superset of `ezkl_lib` so functions contained within `ezkl_lib` will be contained in `ezkl`.

### `ezkl`

#### Utilities


##### `export`

**Description**
The export function is designed to export a PyTorch model in ONNX format for further use in different environments without Python dependencies. It also saves a sample input in JSON format. The function supports models with single input and output.

**Parameters**
`torch_model (required)`: This is the PyTorch model you want to export. It should be an instance of a class that inherits from torch.nn.Module.

`input_shape (optional)`: This is a list of integers specifying the shape of the input tensor that the model expects. For example, for a 2D image, this might be [3, 224, 224] for three color channels (RGB) each of 224x224 pixels. This argument is required if input_array is not provided.

`input_array (optional)`: This is a tensor that you can pass in if you want to specify the exact values of the input tensor. This should be a NumPy ndarray or something that can be converted into one (like a list or tuple of numbers). This argument is required if input_shape is not provided.

`onnx_filename (optional)`: This is the name of the ONNX file that will be generated. The default value is "network.onnx".

`input_filename (optional)`: This is the name of the JSON file that will be generated containing a sample input for the model. The default value is "input.json".

**Returns**
The function does not have a return value. However, it writes two files to the disk:

An ONNX file containing the exported model. The name of the file is specified by the `onnx_filename` argument.
A JSON file containing a sample input for the model. The name of the file is specified by the `input_filename` argument.

**Notes**
The `torch.onnx.export` function is used to convert the PyTorch model to ONNX format. The function requires an example input tensor, which is used to run a forward pass of the model. This is needed because the ONNX exporter needs to know the shapes and data types of the tensors that flow through the model.

The exported ONNX model includes the weights of the trained model and also the network architecture. This means that the model can be used for inference in an environment where PyTorch is not installed.

The `ezkl_lib.forward` function is used for the forward operation to quantize inputs, there may be quantization errors associated with the quantization. Error metric functions can be used to compare the performance before and after quantization.

The function raises an error if neither `input_shape` nor `input_array` are provided, or if both are provided but `input_shape` doesn't match the shape of `input_array`.


### `ezkl_lib`

#### Command Bindings


##### `PyRunArgs`

**Description**

The `PyRunArgs` struct is a Python-friendly data structure that provides a set of arguments required to perform certain operations in a Rust environment. The structure is defined using the pyclass macro from the pyo3 library which makes it compatible with Python via PyO3/Maturin. The fields of PyRunArgs are accessible from Python, and can be both read and modified like Python classes.

**Fields**

`tolerance`: This is an instance of the Tolerance enum, which defines the acceptable range of values for the snark computation.

`scale`: This is a 32-bit unsigned integer which could be used to scale computation.

`bits`: This is a usize type and denotes the bit length for the snark.

`logrows`: This is a 32-bit unsigned integer. This corresponds to the K value used in generating the SRS (Structured Reference String). You can obtain [generated SRS from the powers of tau repository](https://github.com/privacy-scaling-explorations/perpetualpowersoftau) or call the `gen_srs` function for development use.

`public_inputs`: This is a boolean flag indicating whether inputs are public.

`public_outputs`: This is a boolean flag indicating whether outputs are public.

`public_params`: This is a boolean flag indicating whether parameters are public.

`pack_base`: This is a 32-bit unsigned integer. This value refers to the packing base value to be used in the snark.

`batch_size`: This is a 32-bit unsigned integer that specifies the batch size for certain operations.

`allocated_constraints`: This is an optional field that may contain a usize value representing the number of allocated constraints.

**Methods**
The PyRunArgs struct has a new method that provides default instantiation. It initializes the fields with default values.

There's also a conversion method provided for transforming a PyRunArgs instance to a RunArgs instance mainly used internally within the rust python bindings.

**Notes**
For integer fields, you are able to use default Python integers.

Fields marked with the `#[pyo3(get, set)]` attribute can be accessed (read or modified) directly from Python.


##### `table`

**Description**

The table function reads an ONNX model file, and then outputs the nodes of the model as a string.

**Parameters**
`model (required)`: This is a string representing the file path of the ONNX model to be loaded.

`py_run_args (optional)`: This is an instance of PyRunArgs struct. If not provided, a new instance of PyRunArgs with default parameters will be created.

**Returns**
The function returns the table formatted as a string in Python. It may throw an `IOError` if there are issues loading the model file.

**Example**
The following is a Python example of how you might call this function:

```python
import ezkl

run_args = ezkl.PyRunArgs()
run_args.scale = 17

try:
    table_string = ezkl.table("path/to/model", run_args)
    print(table_string)
except Exception as e:
    print(f"An error occurred: {e}")
```


##### `gen_srs`

**Description**
The `gen_srs` function is designed to generate the Structured Reference String (SRS) for the KZGCommitmentScheme<Bn256> and save it to the specified path. The SRS is used in zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge), to set up public parameters.

**Important Note: The SRS generated here is not meant for production**. You should use the [powers of tau repo](https://github.com/privacy-scaling-explorations/perpetualpowersoftau) instead for production applications.

**Parameters**

`params_path (required)`: This is a string representing the file path where the generated SRS will be saved.

`logrows (required)`: This is an integer that represents the K value used in the SRS. More complex models will require more logrows in general. The downside of having more logrows is that the compute resources required will increase, increasing the time taken to generate proofs.

ezkl allows for the overflow of columns which would allow for more complex models to be computer with lesser logrows.

**Returns**
The function returns nothing. Should it be successful a SRS file will be generated at the specified path.

**Example**

The following is a Python example of how you might call this function:

```python
import ezkl

ezkl.gen_srs("path/to/save/params", 17)
print("SRS generation successful.")
```


##### `forward`

**Description**

The forward function runs the forward pass operation for a specified model using the given data. It quantizes the input data, runs the model's forward pass with the quantized data, dequantizes the model output according to the scale factor, and saves the resulting data to a file.

**Parameters**

`data (required)`: A string representing the file path of the data to be processed.

`model (required)`: A string representing the file path of the model to be used for the forward pass operation.

`output (required)`: A string representing the file path where the result of the forward pass operation will be saved.

`py_run_args (optional)`: This is an instance of PyRunArgs struct. If not provided, a new instance of PyRunArgs with default parameters will be created.

**Returns**

The function returns nothing, but creates the output file at the provided `output` path. If there are errors reading and writing to files `IOError` will be raised.

**Example**
The following is a Python example of how you might call this function:

```python
import ezkl

run_args = ezkl.PyRunArgs()

try:
    ezkl.forward("path/to/data", "path/to/model", "path/to/save/output", run_args)
    print("Forward operation successful.")
except Exception as e:
    print(f"An error occurred: {e}")
```


##### `mock`

**Description**

The mock function creates a mock prover using the provided model and data. It processes the model, prepares public inputs, and uses a mock prover to verify the given setup. This function is typically used for testing and validation purposes.

**Parameters**
`data (required)`: A string representing the file path of the data to be used in the mock prover.

`model (required)`: A string representing the file path of the model to be used by the mock prover.

`py_run_args (optional)`: This is an instance of the PyRunArgs struct. If not provided, a new instance of PyRunArgs with default parameters will be created.

**Returns**

The function returns a boolean value. On successful execution, `True` or `False` if it does not. It may return an `IOError` if there are problems reading and writing to files, or a `RuntimeError` if there are problems constructing the circuit.

**Example**
The following is a Python example of how you might call this function:

```python
import ezkl

run_args = ezkl.PyRunArgs()
run_args.scale = 17

try:
    result = ezkl.mock("path/to/data", "path/to/model", run_args)
    if result:
        print("Mock prover verification successful.")
    else:
        print("Mock prover verification failed.")
except Exception as e:
    print(f"An error occurred: {e}")
```


##### `setup`

**Description**

The setup function is used to set up the proving system. It processes a model, constructs a circuit, loads the parameters for the prover, creates and saves the prover and verifier keys, and finally saves the circuit parameters. You will have to run `setup` prior to running the `prove` step.

**Parameters**

`model (required)`: A string representing the file path of the ONNX model to be processed and used to build the circuit.

`vk_path (required)`: A string representing the file path where the verifier key will be saved.

`pk_path (required)`: A string representing the file path where the prover key will be saved.

`params_path (required)`: A string representing the file path where the parameters for the prover will be loaded from.

`circuit_params_path (required)`: A string representing the file path where the circuit parameters will be saved.

`py_run_args (optional)`: This is an instance of the PyRunArgs struct. If not provided, a new instance of PyRunArgs with default parameters will be created.

**Returns**

The function returns a boolean. If all the operations are successful, it will return `True`. If an error occurs at any stage, it will throw an `IOError` for errors associated with reading and writing or a `RuntimeError` associated with failure in the circuit setup.

**Example**

The following is a Python example of how you might call this function:

```python
import ezkl
import os

# Create the PyRunArgs
run_args = ezkl.PyRunArgs()
run_args.scale = 17

# Define the paths
model_path = os.path.join("path/to/model")
vk_path = os.path.join("path/to/vk")
pk_path = os.path.join("path/to/pk")
params_path = os.path.join("path/to/params")
circuit_params_path = os.path.join("path/to/circuit_params")

try:
    result = ezkl.setup(model_path, vk_path, pk_path, params_path, circuit_params_path, run_args)
    if result:
        print("Prover setup successful.")

except Exception as e:
    print(f"An error occurred: {e}")
```


##### `prove`

**Description**

The `prove` function is used to execute a proving operation on a set of inputs. It prepares the data, loads the model circuit parameters, creates a circuit, prepares the public inputs, loads the parameters for the prover, loads the proving key, then based on the strategy, it creates a proof using a circuit and saves the proof.

**Parameters**

`data (required)`: A string representing the file path of the data to be used in the proof.

`model (required)`: A string representing the file path of the ONNX model.

`pk_path (required)`: A string representing the file path where the prover key will be loaded from.

`proof_path (required)`: A string representing the file path where the proof will be saved.

`params_path (required)`: A string representing the file path where the parameters for the prover will be loaded from.

`transcript (required)`: A string representing the type of transcript to be used. `blake`, `poseidon`, or `evm` is supported.

`strategy (required)`: A string representing the strategy to be used for creating the proof. `single` or `accum` is supported.

`circuit_params_path (required)`: A string representing the file path where the circuit parameters will be loaded from.

**Returns**

The function returns a boolean. If all the operations are successful, it will return `True`. If an error occurs at any stage, it will throw an `IOError` for errors associated with reading and writing or a `RuntimeError` associated with failure in the circuit creation.

**Example**

```python
import ezkl
import os

# Define the paths
data_path = os.path.join("path/to/data")
model_path = os.path.join("path/to/model")
proof_path = os.path.join("path/to/prove")
pk_path = os.path.join("path/to/pk")
params_path = os.path.join("path/to/params")
circuit_params_path = os.path.join("path/to/circuit_params")

try:
    result = ezkl.prove(
        data_path,
        model_path,
        pk_path,
        proof_path,
        params_path,
        "poseidon",
        "single",
        circuit_params_path
    )

    if result:
        print("Proof successful.")

except Exception as e:
    print(f"An error occurred: {e}")
```


##### `verify`

**Description**

The verify function is used to verify a given proof. It loads the model circuit parameters, the parameters for the verifier, the proof, and the verifier key. Then it verifies the proof using a circuit.


**Parameters**

`proof_path (required)`: A string representing the file path where the proof will be loaded from.

`circuit_params_path (required)`: A string representing the file path where the circuit parameters will be loaded from.

`vk_path (required)`: A string representing the file path where the verifier key will be loaded from.

`params_path (required)`: A string representing the file path where the parameters for the verifier will be loaded from.

**Returns**

The function returns a boolean. If all the operations are successful, it will return `True`. If an error occurs at any stage, it will throw an `IOError` for errors associated with reading and writing or a `RuntimeError` associated with failure in the circuit creation.

**Example**

```python
import ezkl
import os

# Define the paths
proof_path = os.path.join("path/to/prove")
circuit_params_path = os.path.join("path/to/circuit_params")
vk_path = os.path.join("path/to/vk")
params_path = os.path.join("path/to/params")

try:
    result = ezkl.verify(
        proof_path,
        circuit_params_path,
        vk_path,
        params_path,
    )

    if result:
        print("Verify successful.")

except Exception as e:
    print(f"An error occurred: {e}")
```


##### `aggregate`

**Description**

The `aggregate` function is used to create an aggregated proof from multiple proofs. It loads the parameters, then for each pair of proof and verifier key path, and circuit parameters path, it loads the model circuit parameters, the parameters for the prover, and the verifier key. Then it loads the proofs. Afterwards, it creates an aggregation circuit, the keys for it, creates a proof using the circuit, and saves the proof and the verifier key.

Aggregation helps in making the overall proof size smaller, however, this comes at the expense of additional computational time required. Note that you will usually need more logrows for aggregation, so if proof aggregation is desired use a higher value of K for the logrows used.

**Parameters**

`proof_path (required)`: A string representing the file path where the aggregated proof will be saved.

`aggregation_snarks (required)`: A list of strings representing the file paths where the proofs to be aggregated will be loaded from.

`circuit_params_paths (required)`: A list of strings representing the file paths where the circuit parameters will be loaded from.

`aggregation_vk_paths (required)`: A list of strings representing the file paths where the verifier keys will be loaded from.

`vk_path (required)`: A string representing the file path where the aggregated verifier key will be saved.

`params_path (required)`: A string representing the file path where the parameters for the prover will be loaded from.

`transcript (required)`: A string representing the type of transcript to be used. `blake`, `poseidon`, or `evm` is supported.

`logrows (required)`: An integer specifying the number of logrows available for compute, this will need to correspond to the logrows with the `params_path`.

`check_mode (required)`: A string indicating whether checks will be performed. `safe` or `unsafe` is supported. If safety is not required in the case of development you may use `unsafe` which can provide some speedup.

**Returns**

The function returns a boolean. If all the operations are successful, it will return `True`. If an error occurs at any stage, it will throw an `IOError` for errors associated with reading and writing or a `RuntimeError` associated with failure in the circuit creation.

**Example**

```python
import ezkl
import os

aggregate_proof_path = os.path.join("path/to/aggregate_proof")
proof_path = os.path.join("path/to/proof")
circuit_params_path = os.path.join("path/to/circuit_params")
aggregate_vk_path = os.path.join("path/to/vk")
params_path = os.path.join("path/to/params")

res = ezkl_lib.aggregate(
    aggregate_proof_path,
    [proof_path],
    [circuit_params_path],
    [vk_path],
    aggregate_vk_path,
    params_path,
    "poseidon",
    20,
    "unsafe"
```


##### `verify_aggr`

**Description**

The `verify_aggr` function is used to verify an aggregated proof. It loads the parameters, the proof, and the verifier key, then uses these to verify the proof using the Accumulator Strategy. If the verification is successful, it returns true; otherwise, it returns false.

**Parameters**

`proof_path (required)`: A string representing the file path where the aggregated proof will be loaded from.

`vk_path (required)`: A string representing the file path where the verifier key will be loaded from.

`params_path (required)`: A string representing the file path where the parameters for the verifier will be loaded from.

`logrows (required)`: An integer specifying the number of logrows used for computation.

**Returns**

If the verification of the proof is successful, it will return `True`. If the proof cannot be verified or an error occurs at any stage, it will return `False`. If an error occurs at any stage, it will throw an `IOError` for errors associated with reading and writing or a `RuntimeError` associated with failure in the circuit creation.

**Example**

```python
import ezkl
import os

aggregate_proof_path = os.path.join("path/to/aggregate_proof")
aggregate_vk_path = os.path.join("path/to/vk")
params_path = os.path.join("path/to/params")

res = ezkl.verify_aggr(
    aggregate_proof_path,
    aggregate_vk_path,
    params_path,
    20,
)
```

##### `create_evm_verifier`

**Description**

The `create_evm_verifier` function is used to generate an Ethereum Virtual Machine (EVM) compatible verifier. This function requires that the Solidity compiler (solc) is installed in the user's environment. You should use `solc-select` or `svm-rs` to help manage solc installed in the user's environment.

It first loads the model circuit parameters, the verifier parameters, and the verifier key. Using these, it generates the EVM compatible verifier. The verifier code is then saved as Yul and/or Solidity code.

**Parameters**

`vk_path (required)`: A string representing the file path where the verifier key will be loaded from.

`params_path (required)`: A string representing the file path where the parameters for the verifier will be loaded from.

`circuit_params_path (required)`: A string representing the file path where the circuit parameters will be loaded from.

`deployment_code_path (required)`: A string representing the file path where the yul deployment code for the verifier will be saved.

`sol_code_path (optional)`: A string representing the file path where the Solidity code for the verifier will be saved.


**Returns**

If the EVM compatible verifier is successfully generated and saved, it will return `True`. If an error occurs at any stage, it will return an `IOError` or `RuntimeError`.

**Example**

```python
import ezkl
import os

vk_path = os.path.join(folder_path, 'test_evm.vk')
circuit_params_path = os.path.join(folder_path, 'circuit.params')
deployment_code_path = os.path.join(folder_path, 'deploy.code')
sol_code_path = os.path.join(folder_path, 'test.sol')

res = ezkl.create_evm_verifier(
    vk_path,
    params_path,
    circuit_params_path,
    deployment_code_path,
    sol_code_path
)

assert res == True
assert os.path.isfile(deployment_code_path)
assert os.path.isfile(sol_code_path)
```


##### `verify_evm`

**Description**

The `verify_evm` function verifies an Ethereum Virtual Machine (EVM) compatible proof. The function requires the Solidity compiler (solc) to be installed in the user's environment. The proof is loaded, and the deployment code is loaded from the given path. The proof is then verified using the EVM and optionally verified using Solidity if a path to the Solidity code is provided.

**Parameters**

`proof_path (required)`: A string representing the file path where the proof will be loaded from.

`deployment_code_path (required)`: A string representing the file path where the deployment code will be loaded from.

`sol_code_path (optional)`: A string representing the file path where the Solidity code will be loaded from. If provided, the proof will also be verified using this Solidity code.

`runs (optional)`: An integer > representing the number of times to run the Solidity verifier. This is only used if sol_code_path is provided.

**Returns**

If the EVM compatible proof is successfully verified, it will return `True`. If an error occurs at any stage, it will return an Err variant containing the Python error PyErr. If an error occurs at any stage, it will return an `IOError` or `RuntimeError`.

**Example**

```python
import ezkl
import os

proof_path = os.path.join(folder_path, 'test_evm.pf')
deployment_code_path = os.path.join(folder_path, 'deploy.code')


res = ezkl.verify_evm(
    proof_path,
    deployment_code_path,
)

assert res == True
```


##### `create_evm_verifier_aggr`

**Description**

The `create_evm_verifier_aggr` function creates an EVM compatible aggregate verifier. The function requires the Solidity compiler (solc) to be installed in the user's environment. The function loads the parameters and verifier key and generates the aggregate verifier. The generated verifier is then saved as Yul code, and optionally as Solidity code if a path to save the Solidity code is provided.

**Parameters**
`vk_path (required)`: A string representing the file path where the verifier key will be loaded from.

`params_path (required)`: A string representing the file path where the parameters for the verifier will be loaded from.

`deployment_code_path (required)`: A string representing the file path where the Yul deployment code for the verifier will be saved.

`sol_code_path (optional)`: A string representing the file path where the Solidity code for the verifier will be saved.

**Returns**

The function returns a boolean. If the EVM compatible aggregate verifier is successfully created and saved, it will return `True`. If an error occurs at any stage, it will return a `IOError` or `RuntimeError`.

**Example**

```python
import ezkl
import os

data_path = os.path.join(
    examples_path,
    'onnx',
    '1l_relu',
    'input.json'
)

model_path = os.path.join(
    examples_path,
    'onnx',
    '1l_relu',
    'network.onnx'
)

pk_path = os.path.join(folder_path, '1l_relu.pk')
vk_path = os.path.join(folder_path, '1l_relu.vk')
circuit_params_path = os.path.join(folder_path, '1l_relu_circuit.params')

ezkl_lib.setup(
    model_path,
    vk_path,
    pk_path,
    params_path,
    circuit_params_path,
)

proof_path = os.path.join(folder_path, '1l_relu.pf')

ezkl_lib.prove(
    data_path,
    model_path,
    pk_path,
    proof_path,
    params_path,
    "poseidon",
    "accum",
    circuit_params_path,
)

aggregate_proof_path = os.path.join(folder_path, 'aggr_1l_relu.pf')
aggregate_vk_path = os.path.join(folder_path, 'aggr_1l_relu.vk')

res = ezkl_lib.aggregate(
    aggregate_proof_path,
    [proof_path],
    [circuit_params_path],
    [vk_path],
    aggregate_vk_path,
    params_k20_path,
    "evm",
    20,
    "unsafe"
)

assert res == True
assert os.path.isfile(aggregate_proof_path)
assert os.path.isfile(aggregate_vk_path)

aggregate_deploy_path = os.path.join(folder_path, 'aggr_1l_relu.code')
sol_code_path = os.path.join(folder_path, 'aggr_1l_relu.sol')

res = ezkl_lib.create_evm_verifier_aggr(
    aggregate_vk_path,
    params_k20_path,
    aggregate_deploy_path,
    sol_code_path
)

assert res == True
assert os.path.isfile(aggregate_deploy_path)

res = ezkl_lib.verify_aggr(
    aggregate_proof_path,
    aggregate_vk_path,
    params_k20_path,
    20,
)
assert res == True
```


##### `print_proof_hex`

**Description**

The `print_proof_hex` function loads a proof from a given file path and returns a string containing the hexadecimal representation of the proof.

**Parameters**

`proof_path (required)`: A string representing the file path where the proof will be loaded from.

**Returns**

If the proof is successfully loaded and converted to a hexadecimal string, it will return a hex string in Python.
