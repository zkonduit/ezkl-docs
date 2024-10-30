---
icon: bug
order: 86
---
![](../assets/copter.png)

# Troubleshooting

### What programming languages and frameworks does ezkl support?

`ezkl` is a command line tool, and a library that can be used from Rust or Python. You may want to use Python to create a neural network and export it. Though ezkl is built with Rust, you do not need to use Rust except possibly for installation.

### Do I need to know Rust before getting started with ezkl?

No, Rust is not a requirement to use the library. As long as you have the ONNX file and proper input & output format of the model, you can use `ezkl`.

# Technical

### Why is the gen-srs step slow?

Generating a structured reference string takes a considerable amount of time and memory. Make sure your machine has enough memory available and wait for the process to finish. Alternatively, download a pre-generated srs using `get-srs`. This is both safer and faster.

### Can I use ezkl with other machine learning frameworks like TensorFlow, PyTorch, or Scikit-learn?

All `ezkl` requires is an onnx file and a JSON configuration of mock inputs and outputs of the neural network. At this time, it works best with PyTorch.

### How fast is ezkl?

We believe that `ezkl` is the fastest zkml package available, and we are working hard every day to make it faster. Feel free to run `cargo bench` on your machine to see what the benchmarks are for your hardware.

### Do I need to deploy a verifier smart contract to use ezkl?

No. We recently integrated a [WASM verifier](https://github.com/zkonduit/ezkl/pull/219) that you can use to verify proofs from your web application. You can also use the EVM verifier to verify proofs locally, or the command line `ezkl verify` command.

# Errors

### Error: VerifyError

A VerifyError is thrown when the Mock prover fails, often due to a mismatched shape problem in the model. Please verify that your input.json inputs and outputs match those of your .onnx file.

### Error: DimMismatch

A DimMismatch error is thrown when there is a mismatch in the lengths of the tensor operands during circuit construction.

### Error: LookupInstantiation

This error is thrown when there is an error during the creation of a lookup table.

### Error: TableAlreadyAssigned

A TableAlreadyAssigned Error is thrown when `ezkl` attempts to initialize a lookup table that has already been initialized.

### Error: UnsupportedOp

An UnsupportedOp Error is thrown when there is an operation in the ONNX file that `ezkl` cannot yet handle. Please look at the supported operations under src/circuit/ops to get an idea of what operations `ezkl` can handle.

### Error: PyValueError

This is a pyo3 error that occurs when a data type fails to be extracted from Python to Rust. Please make sure you are passing the correct data types when utilizing the python bindings.

### Error: InvalidLookupInputs

InvalidLookupInputs is thrown when the wrong inputs were passed to a lookup node.

### Error: InvalidDims

InvalidDims is thrown when there is a shape mismatch in circuit construction. Invalid dimensions were used for a node with the given index and description.

### Error: WrongMethod

This error means that the wrong method was called to configure a node with the given index and description.

### Error: MissingNode

MissingNode is thrown when a requested node is missing in the graph with the given index.

### Error: OpMismatch

OpMismatch is thrown when an unsupported method was called on a node with the given index and description.

### Error: UnsupportedOp

UnsupportedOp is thrown when there is an operation in the onnx graph that isn't supported by `ezkl`.

### Error: MissingParams

MissingParams is thrown when a node has missing parameters; please check the parameters in your model's operations.

### Error: MisformedParams

MisformedParams is thrown when a node has misformed parameters; the error can stem from erroneous padding height and width dimensions, wrong kernel / data format, dilations that are not uint type, and more.

### Error: Visibility

This error is typically thrown when no public variables are passed to the circuit configuration function.

### Error: NonConstantDiv

`ezkl` only supports divisions by constants.

### Error: NonConstantPower

`ezkl` only supports constant exponents.

### Error: RescalingError

This error is thrown when attempting to rescale inputs for an operation.

### Error: ModelLoad

This error is thrown when a model fails to load. Please check your onnx file for missing connections / unsupported layers. We suggest using [Netron](https://netron.app/) to view onnx files.
