# General
### What programming languages and frameworks does EZKL support?
Ezkl does not require application of a programming language. You may want to use Python to create a neural network and export it with pyezkl, but though the library is built with Rust, you do not need to use Rust. 

# Technical
### Why is the gen-srs step is stalling?
Generating a structured reference string takes a considerable amount of time and memory. Make sure your machine has enough memory available and wait for the process to finish.

### Can I use EZKL with other machine learning frameworks like TensorFlow, PyTorch, or Scikit-learn?
All ezkl requires is an onnx file and a JSON configuration of mock inputs and outputs of the neural network.

### How fast is ezkl?
Feel free to run 'cargo bench' on your machine to see what the benchmarks are for your hardware. //Include more info about general machine usage

# Errors:

### Error: VerifyError
A VerifyError is thrown when the Mock prover fails due to a mismatched shape problem in the model. Please verify that your input.json inputs and outputs match those of your .onnx file. 

### Error: DimMismatch
### Error: LookupInstantiation
### Error: TableAlreadyAssigned
### Error: UnsupportedOp