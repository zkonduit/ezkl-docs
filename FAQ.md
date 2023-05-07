---
icon: question
order: 1
---
# General
### What programming languages and frameworks does ezkl support?
Ezkl does not require application of a programming language. You may want to use Python to create a neural network and export it with pyezkl, but though the library is built with Rust, you do not need to use Rust. 

### Do I need to know Rust before getting started with ezkl?
No, Rust is not a requirement to use the library. As long as you have the ONNX file and proper input & output format of the model, you can use ezkl. 


# Technical
### Why is the gen-srs step is stalling?
Generating a structured reference string takes a considerable amount of time and memory. Make sure your machine has enough memory available and wait for the process to finish.

### Can I use ezkl with other machine learning frameworks like TensorFlow, PyTorch, or Scikit-learn?
All ezkl requires is an onnx file and a JSON configuration of mock inputs and outputs of the neural network.

### How fast is ezkl?
Feel free to run 'cargo bench' on your machine to see what the benchmarks are for your hardware.

### Do I need to deploy a verifier smart contract to use ezkl?
No, we recently integrated a (WASM verifier)[https://github.com/zkonduit/ezkl/pull/219] that you can use to verify proofs from your web application. You can also use the EVM verifier to verify proofs locally.

# Errors:
### Error: VerifyError
A VerifyError is thrown when the Mock prover fails due to a mismatched shape problem in the model. Please verify that your input.json inputs and outputs match those of your .onnx file. 

### Error: DimMismatch
A DimMismatch error is thrown when there is a mismatch in the lengths of the tensor operands during circuit construction. 
### Error: LookupInstantiation
This error is thrown when there is an error during the creation of a lookup table
### Error: TableAlreadyAssigned
A TableAlreadyAssigned Error is thrown when ezkl attempts to initialize a lookup table that has already been initialized
### Error: UnsupportedOp
An UnsupportedOp Error is thrown when there is an operation in the ONNX file that ezkl cannot yet handle. Please look at the supported operations under src/circuit/ops to get an idea of what operations ezkl can handle.
