---
order: 4
---

# MNIST Model Tutorial

This is part 2 of our tutorial on building the [e2e-mnist](https://e2e-mnist.vercel.app) demo app where we go over the model training and exporting process. To follow along with this portion of the tutorial, you can run the associated [notebook](https://github.com/zkonduit/e2e-mnist/blob/main/mnist_classifier.ipynb) in Google Colab.

[!embed aspect="1:1" height="340"](https://www.researchgate.net/publication/318972455/figure/fig2/AS:525282893615105@1502248609221/The-overall-LeNet-architecture-The-numbers-at-the-convolution-and-pooling-layers.png)

> Diagram of the LeNet architecture. Image source: [ResearchGate](https://www.researchgate.net/profile/Gerard-Pons-3/publication/318972455/figure/fig2/AS:525282893615105@1502248609221/The-overall-LeNet-architecture-The-numbers-at-the-convolution-and-pooling-layers.png)


### Data Preparation and Training

1. Dataset Loading:

-   The MNIST dataset, a collection of handwritten digits, is loaded using PyTorch's torchvision.datasets.MNIST class. This process involves specifying parameters such as root for the storage directory, train to toggle between training and testing sets, transform to apply transformations like converting images to tensor format, and download to download the data if it's not already present locally.

2. Normalization Function:

-   The normalize_img function is a crucial part of the data preparation. 
-   It rounds the pixel values of the images to 0 or 1. This step mimics th binary nature of the input data expected from a drawing interface, where the pixels are either filled or not. We will build this out in part 4 of the tutorial where we focus on the frontend.

3. Data Pipeline for Training Set:
-   The training dataset undergoes normalization using the DataLoader class, which combines the dataset and a sampler, providing an iterable over the dataset.
-   Data is batched with a size of 256, meaning 256 images are processed in each iteration of training.
-   No explicit shuffling or prefetching is used here, but these could be incorporated for enhanced performance.
4. Data Pipeline for Testing Set:
-   The testing dataset is similarly normalized and batched using the DataLoader.
-   The batching process for the test dataset mirrors the training set, facilitating a consistent evaluation process.

### Training

1. Model Building:

-   The LeNet model, used for this tutorial, is defined using PyTorch’s neural network module (torch.nn). It includes a series of convolutional layers (nn.Conv2d) and fully connected layers (nn.Linear), with activations functions (F.sigmoid) applied appropriately. 
-   The architecture is a classic choice for image classification tasks.

2. Model Configuration:
-   The model is moved to the appropriate device (GPU if available, otherwise CPU) using PyTorch’s .to(device) method.
-   An instance of the Adam optimizer is created with the model's parameters. Adam is chosen for its efficiency in handling sparse gradients and adaptive learning rate capabilities.
-   The loss function is defined as CrossEntropyLoss, a standard choice for classification tasks with multiple classes.

3. Model Training:
-   The training loop runs for a predefined number of epochs (25 in this case). Each epoch consists of a forward pass where predictions are generated, a loss calculation, and a   backward pass for gradients computation and parameters update.
-   The accuracy of the model is evaluated at the end of each epoch on the test dataset. This provides insight into the model's generalization capability on unseen data.

### Exporting to ONNX

The trained PyTorch model is then converted to the ONNX (Open Neural Network Exchange) format using torch.onnx.export, which facilitates model interoperability and makes it compatible with various platforms and tools. The conversion process involves specifying the input shape and the ONNX opset version (always set to 12). Alongside, an input data file (input.json) is created by extracting a sample dataset from the training dataset and serializing it into JSON format. This data is used later for EZKL hub deployment.

### Deploying to the EZKL Hub

If you have been following along with the `mnist_classifier` notebook, you should be on cell number [7](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#scrollTo=dS-yXte30rZ3). There is another version of this notebook on the main EZKL repo where we perform the setup and proving all locally [here](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#scrollTo=dS-yXte30rZ3). Comparing this version of the notebook to the one on the e2e-mnist repo, you may notice the cells are identical up until cell 7. After this point we generate a lot of zk specific artifacts (namely a settings file, a compiled circuit, a proving key, a verification key, a witness, a proof and solidity verifier) whereas with the other notebook we only have one more cell that calls into the hub. This is a lot of artifacts to manage and takes a lot of compute resources to generate. Moreover, once you have the artifacts necessary to generate proofs, you then have to set up a proving server if you want to generate proofs on behalf of your users. 

To make this process easier, we have created a backend proving service called [EZKL Hub](https://app.ezkl.xyz/) that generates and manages these artifacts and makes serving proofs as easy as calling a graph QL endpoint. To deploy your model to the hub, you will need to sign into EZKL Hub using your Github. 

Once you approve EZKL Hub access to your Github openid profile, modify cell [7](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#scrollTo=dS-yXte30rZ3) by adding your Github username to the `test_hub_name` variable.

Specifically we:

1. Initialize the EZKL run args: These are the arguments that determine the visibility of the inputs, parameters and outputs of the model. We set the input visibility to private, the parameter visibility to fixed and the output visibility to public. This means that the inputs to the model will be private to the verifier, the parameters will be set as fixed columns in the circuit (meaning that once the circuit has been created they are hardcoded into the cicuit as fixed columns, so they can't be updated later) and the outputs will be visible to the verifier. 

2. Gather Calibration Data: We gather a set of data points from the training set to use for calibration. Calibrating the circuit with sample inputs like this ensures that the lookup tables within our circuit are robust to outliers and therefore not likely to fall out of range of the [lookups](https://zcash.github.io/halo2/design/proving-system/lookup.html) (we represent non-linearities as lookup tables).  

3. Deploy the Model to the Hub: We deploy the model to the hub using the `create_hub_artifact` function. This function takes in the model path, the data path, the name of the model, the organization id, the calibration data path, the target directory and the run args. The model path is the path to the ONNX model we exported in the previous section. The input data is used to ensure that all of the constraints of the model hold. We set the calibration target to `resources` so that circuit settings favor a more cost efficient proving. You can also set the target to `accuracy` if you want to favor a more accurate circuit (accurate in the sense that the outputs are closer to the outputs of the canonical onnx model) at the expense of increased proving time.

4. Test proof: Now that we have generated our artifacts, we can generate proofs against them on the hub to ensure that the our model was deployed properly. In part 4 of this tutorial when we start building out our client we will be calling this same EZKL hub endpoint to generate proofs on behalf of our users.
