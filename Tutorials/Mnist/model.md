---
order: 3
---

# MNIST Model Tutorial

This is part 2 of our tutorial on building the [e2e-mnist] (https://e2e-mnist.vercel.app) demo app where we go over the model training and exporting process. To follow along with portion of the tutorial, you can run the associated [notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#) in Google Colab.

### Data Preparation

1. Dataset Loading: The MNIST dataset, containing handwritten digits, is loaded using TensorFlow's `tfds.load method`. It splits the data into training (`ds_train`) and testing (`ds_test`) sets, with shuffling enabled for variety and `as_supervised` set to `True` to load the data in a format suitable for supervised learning. The `with_info parameter` is set to `True` to get detailed information about the dataset.

2. Normalization Function: The central element of this data preparation process is the normalization function, `normalize_img`. This function converts the image data from the `uint8` data type to `float32`, subsequently scaling the pixel values to fall within a range between 0 and 1. After scaling, these values are rounded to the nearest integers. This rounding step is specifically designed to mirror the binary nature of the input data, which originates from drawn digits on the grid of the front-end application. 

3. Data Pipeline for Training Set:

- The training dataset is passed through the normalize_img function using the map method for normalization.
- `cache` method is used to cache the data in memory for faster access during training.
- The dataset is shuffled using the total number of examples in the training set, ensuring randomization which is important for effective learning.
- The data is batched into sizes of 128, which means that 128 images will be fed into the model at a time during training.
- `prefetch` is used to prepare the data loading for the next batch while the current batch is being used for training, improving efficiency.

4. Data Pipeline for Training Set:

- Similar to the training set, the testing dataset is also normalized using the `map` method.
- The data is batched into sizes of 128, like the training set.
- `cache` and `prefetch` are used for efficient data access and loading, similar to the training set.

### Training

1. Model Building: The model is built using TensorFlow's Keras API. It is a sequential model, which means the layers are arranged in a linear stack. The first layer is a Flatten layer that transforms the input data (images of 28x28 pixels) into a one-dimensional array. Following this, there is a Dense layer with 128 neurons (units) and ReLU (rectified linear unit) activation, a popular choice for non-linear transformation. The final layer is another Dense layer with 10 neurons, corresponding to the 10 digit classes in the MNIST dataset.

2. Model Compilation: In the compilation step, the model is configured for training. The Adam optimizer is used with a learning rate of 0.001, which is a common choice for training neural networks due to its efficiency and adaptiveness. The loss function is set to SparseCategoricalCrossentropy with logits (the raw output of the last layer) to compute the cross-entropy loss between the labels and predictions, a typical choice for multi-class classification tasks. The metric used to evaluate the model's performance during training and testing is SparseCategoricalAccuracy.

3. Model Training: The model is trained using the fit method on the ds_train dataset. Training runs for 6 epochs, meaning the entire dataset will pass through the network six times. During training, the model's performance is also evaluated on the ds_test dataset, serving as the validation data. This allows for monitoring the model's performance on unseen data and helps in detecting issues like overfitting.

### Exporting and Calibrating the Model

1. Exporting the Model: The trained TensorFlow model is first converted to the ONNX (Open Neural Network Exchange) format using tf2onnx, which facilitates model interoperability and makes it compatible with various platforms and tools. The conversion process involves specifying the input shape and the ONNX opset version. Alongside, an input data file (input.json) is created by extracting a sample dataset from the training dataset and serializing it into JSON format. This data is used later in calibration

2. Calibrating the Circuit with EZKL: The ezkl's core functionality is to compile ONNX models into a ZK circuits. Each circuit has a set of hyper parameter that determines its geometric configuration (shape, size, etc), [lookup tables](https://zcash.github.io/halo2/design/proving-system/lookup.html) params and model visibility. The process includes several steps:

- Setting up the arguments for ezkl's run, specifying the visibility of inputs, parameters, and outputs, and other variable settings.

- Generating calibration data (cal_data.json) by capturing a set of data points from the training set. 

- Utilizing ezkl functions to generate settings (gen_settings), calibrate these settings (calibrate_settings), and compile the circuit (compile_circuit). These steps involve creating a settings file (settings.json), compiling the model and calibrating the model for cost efficient proving. 

3. Circuit Setup: The final step creates the final set of artifacts needed for proof generation and verification. This includes:

- Generating an SRS (Structured Reference String) file (kzg.srs) required because we are using [KZG commitments](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html).
- Setting up the circuit parameters, including the generation of verification (vk_path) and proving (pk_path) keys.
- A witness for our model, which stores the quantized input and corresponding model outputs. After that, we run a mock proof to check that all the constraints are valid.

Make sure to save the `key.pk`, `key.vk`, `kzg.srs`, `settings.json` and `network.compiled` files as all of these artifacts will need to be ported over as assets to the frontend to support proving and verifying of the digit recognition model in the browser.

### Deploying and Verifying the EVM Verifier Locally

At this point we have all of the artifacts we need to enable in browser proof generation and verification. But we are not quite finished yet! To support on chain verifying, we will need to create an Ethereum Smart Contract that acts as a verifier for our model. 

Once generated, we can spin up a local Ethereum node using anvil by opening a new terminal (or using the notebook bash tooling `!`) and entering this command `anvil -p 3030`. 

Once deployed, we can verify the proof on the EVM using the `verify_evm` function from the EZKL library, passing the proof and the contract's address to it. If you followed the steps correctly, the proof should be verified successfully!

### Remix Deployment

Now that we have a confirmation that our solidity verifier works, lets deploy it a public blockchain so that we can interact with it from the browser. In this tutorial we will deploy on the Polygon Mumbai testnet using Remix, but you can deploy to any EVM compatible chain so long as its any version post `Constantinople`. 

If you are deploying on L2, (like we are in the tutorial), we recommend using a solc compiler version that is `>=0.8.20` as it will use the `PUSH0` opcode, an opcode exclusive to `Shanghai` networks.

1. First, we need to copy the contents of the `test.sol` solidity verifier code into the [Remix IDE](https://remix.ethereum.org/).

2. Next, we need to adjust the solidity compiler settings. Make sure to set the EVM version to the same version as the L2 you are deploying to. In our case, we are deploying to Polygon Mumbai, so we set the EVM version to `London` and the compiler to version `0.8.17`. We will also need to enable optimization or else you will get a `stack too deep` compiler error.

3. Once you have aquired some [Mumbai MATIC](https://mumbaifaucet.com/) to cover the deployment cost, you can deploy the contract by changing your environment to `Injected Provider - MetaMask`, selecting the Mumbai network on your Metamask wallet and then clicking the `Deploy` button, confirming the transaction and the voila! Your Digit Recognition Verifier should be deployed.

4. Finally we need to obtain the ABI and address of the contract as we will need those two pieces of information to make calls to it from the browser. We can get the ABI by clicking the `copy ABI` button in the solidiy compiler page of the Remix IDE. We can obtain the address from inspecting the confirmed transaction data in the Remix terminal.

In the next part of the tutorial, we will go over the frontend implementation of the app and how to use the JS bindings to generate proofs in the browser, as well as how to verify them on the EVM and browser. 
