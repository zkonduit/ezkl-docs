---
order: 5
---

# Overview

# **Building a Digit Recognition Classifier and Verifier with PyTorch, EZKL, JS Bindings and Next.js**


Check out the live demo of the app:

[!embed el="embed" aspect="1:1" width="900 height="600"](https://e2e-mnist.vercel.app/)


 The entire app is open source which you can view on Github; check out the model [here](https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb), the frontend [here](https://github.com/zkonduit/e2e-mnist) and the js bindigs npm package [here](https://www.npmjs.com/package/@ezkljs/engine)

## **Introduction**

In this tutorial, we will demonstrate an end-to-end tutorial that leverages both the Python and JS bindings; The python bindings for training a model and setting up a circuit and the JS bindings for generating proofs against the circuit and subsequent verifications in the browser. Specifically, we will build a digit recognition model that can recognize hand drawn digits. 

This entire end to end application is a good faith fork of the first ZKML application built by horacepan, sunfishstanford and henripal as part of 0xPARC's winter 2021 applied zk learning group. You can find the original app [here]( https://zkmnist.netlify.app/) and source code [here](https://github.com/0xZKML/zk-mnist).

## **Model Development**

- Model Building: Employs TensorFlow's Keras API to construct a sequential model tailored for the MNIST dataset.
- Model Compilation: Configures the model with Adam optimizer, SparseCategoricalCrossentropy loss function, and SparseCategoricalAccuracy metric.
- Model Training: Executes training over 6 epochs, evaluating performance on the testing set.

## **Model Exporting and Calibrating**

- Exporting the Model: Converts the trained model to ONNX format and creates a JSON-formatted input data file.
- Calibrating with EZKL: Involves setting up ezkl arguments, selecting calibration data, and compiling the circuit.
- Circuit Setup: Generates necessary artifacts for proof generation and verification.

## **EVM Verifier Deployment and Verification**

- Local Ethereum Node: Spins up a local Ethereum node and employs the `verify_evm` function from the EZKL library for proof verification.
- Solidity Verifier Deployment: Guides through deploying the solidity verifier to the Polygon Mumbai testnet using Remix, including compiler settings adjustment, acquiring Mumbai MATIC, and obtaining the ABI and address of the contract.

## **Frontend and JS Binding**

- Digit Drawing: Provides a React component for users to draw a digit on a 28x28 grid, with real-time grid updates
- React's useState Hook: Manages various states like grid data, proof generation, prediction results, and verification statuses.
- Tensor Preparation: Converts the drawn digit into a tensor for processing.
- Digit Classification: Analyzes the drawn digit and displays the prediction result from the `witness` output.
- Proof Generation: Utilizes `utils.handleGenProofButton` for proving that the model classified a given digit.
- Smart Contract Integration: Uses wagmi's public provider and specifies the contract's ABI and address for on-chain verification.

## **Conclusion**

We hope this tutorial provides a foundation for building more complex production-ready applications on EZKL that require secure, verifiable judgments.
