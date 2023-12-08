---
order: 5
---

# Overview

# **Building a Digit Recognition Classifier and Verifier with PyTorch, EZKL, JS Bindings and Next.js**

Check out the live demo of the app:

[!embed](https://e2e-mnist.vercel.app/)


The entire app is open source which you can view on Github; check out the model [here](https://github.com/zkonduit/e2e-mnist/blob/main/mnist_classifier.ipynb), the frontend [here](https://github.com/zkonduit/e2e-mnist) and the smart contract [here](https://goerli-optimism.etherscan.io/address/0xf5cDCD333E3Fd09929BAcEa32c2c1E3A5A746d45#code)

## **Introduction**

In this tutorial, we will demonstrate an end-to-end tutorial that showcases how to train a model for handrawn digit recognition, creating proof of classification, deploying an EVM verifier, verifying the proof and parsing the associated instances all on on-chain. Specifically, we will build a digit recognition model that can recognize hand drawn digits trained on the MNIST dataset. 

This application is a good faith fork of the first ZKML application built by horacepan, sunfishstanford and henripal as part of 0xPARC's winter 2021 applied zk learning group. You can find the original app [here](https://zkmnist.netlify.app/) and source code [here](https://github.com/0xZKML/zk-mnist). This is the project that inspired the creaton of EZKL and we are excited to replicate and gamify it :).

## **Data Preparation and Training**

-   Dataset Loading: Utilizes PyTorch's torchvision.datasets.MNIST class to load the MNIST dataset of handwritten digits.
-   Normalization Function: Implements normalize_img to adjust pixel values to binary (0 or 1), mimicking input data from a drawing interface.
-   Data Pipeline: Employs DataLoader for normalization and batching (256 images per batch) for both training and testing datasets.

## **Training Process**

-   Model Building: Uses the LeNet model defined in PyTorch with convolutional and fully connected layers.
-   Model Configuration: The model is adapted to the appropriate device (GPU/CPU), using Adam optimizer and CrossEntropyLoss function.
-   Training Loop: Runs for 25 epochs, including forward pass, loss calculation, and backpropagation, with model accuracy evaluated after each epoch.
-   Exporting to ONNX: Converts the PyTorch model to ONNX format for interoperability and compatibility, alongside creating a sample dataset in JSON format for later use.

## **Deploying to EZKL Hub**

-   Proving Service: EZKL Hub, a backend proving service, simplifies the process of generating and managing zk-specific artifacts.
-   Deployment Steps: Includes initializing EZKL run args, gathering calibration data, deploying the model using create_hub_artifact, and testing proofs on the hub.

## **EVM Verifier Deployment and Verification**

-   Verifier Contract Interface: Uses an external verifier to validate proofs. The verifyProof function checks if a submitted proof is valid.
-   Constants and State Variables:
    -   Holds a reference to the Verifier contract.
    -   Constants like `ORDER` and `THRESHOLD` are used for operational purposes.
    -   Mappings like entered, clan, and counts track user submissions and their associated digits.
-   Core Functions: Enables users to submit a digit with a proof, updating submission records.
    -   submitDigit: Enables users to submit a digit with a proof, updating submission records.
    -   feltToInt: Adjusts field elements to integers.
    -   getCounts: Simplifies accessing submission counts
-  Deployment Steps: 
    -   Download Verifier from EZKL Hub: The necessary Verifier contract is available for download on the EZKL Hub.
    -   Adjust Compiler Settings: Set the EVM version to 'London' and enable optimizations to ensure compatibility and avoid errors.
    -   Contract Deployment: Deploy the Verifier first, then use its address for deploying the Mnist Clan contract.

## **Frontend and JS Binding**

- Digit Drawing: Provides a React component for users to draw a digit on a 28x28 grid, with real-time grid updates
- React's useState Hook: Manages various states like grid data, proof generation, prediction results, and verification statuses.
- Tensor Preparation: Converts the drawn digit into a tensor for processing.
- Digit Classification: Analyzes the drawn digit and displays the prediction result from the `proof.instances` output.
- Proof Generation: Utilizes `hub.initiatProof` and `hub.getProof` for proving that the model classified a given digit.
- Smart Contract Integration: Uses wagmi's public provider and specifies the contract's ABI and address for on-chain verification.

## **Conclusion**

We hope this tutorial provides a foundation for building more complex production-ready applications on EZKL that require secure, verifiable judgments.
