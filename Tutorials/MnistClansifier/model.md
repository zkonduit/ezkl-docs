---
order: 4
---

# MNIST Model Tutorial

This is part 2 of our tutorial on building the [e2e-mnist](https://e2e-mnist.vercel.app) demo app where we go over the model training and exporting process. To follow along with this portion of the tutorial, you can run the associated [notebook](https://github.com/zkonduit/e2e-mnist/blob/main/mnist_classifier.ipynb) in Google Colab.

[!embed aspect="1:1" height="340"](https://www.researchgate.net/publication/318972455/figure/fig2/AS:525282893615105@1502248609221/The-overall-LeNet-architecture-The-numbers-at-the-convolution-and-pooling-layers.png)

> Diagram of the LeNet architecture. Image source: [ResearchGate](https://www.researchgate.net/profile/Gerard-Pons-3/publication/318972455/figure/fig2/AS:525282893615105@1502248609221/The-overall-LeNet-architecture-The-numbers-at-the-convolution-and-pooling-layers.png)


### Data Preparation and Training

1. Lenet Model:

-   The LeNet model is defined using PyTorch's neural network module (torch.nn). It includes a series of convolutional layers (nn.Conv2d) and fully connected layers (nn.Linear), with activations functions (F.sigmoid) applied appropriately. The architecture is a classic choice for image classification tasks.

```python mnist_classifier.ipynb
import torch
import torch.nn as nn
import torch.nn.functional as F

class LeNet(nn.Module):
    def __init__(self):
        super(LeNet, self).__init__()
        # Convolutional encoder
        self.conv1= nn.Conv2d(1, 6, 5)  # 1 input channel, 6 output cha nnels, 5x5 kernel
        self.conv2 = nn.Conv2d(6, 16, 5) # 6 input channels, 16 output channels, 5x5 kernel

        # Fully connected layers / Dense block
        self.fc1 = nn.Linear(16 * 4 * 4, 120) 
        self.fc2 = nn.Linear(120, 84)         # 120 inputs, 84 outputs
        self.fc3 = nn.Linear(84, 10)          # 84 inputs, 10 outputs (number of classes)

    def forward(self, x):
        # Convolutional block
        x = F.avg_pool2d(F.sigmoid(self.conv1(x)), (2, 2)) # Convolution -> Sigmoid -> Avg Pool
        x = F.avg_pool2d(F.sigmoid(self.conv2(x)), (2, 2)) # Convolution -> Sigmoid -> Avg Pool

        # Flattening
        x = x.view(x.size(0), -1)

        # Fully connected layers
        x = F.sigmoid(self.fc1(x))
        x = F.sigmoid(self.fc2(x))
        x = self.fc3(x)  # No activation function here, will use CrossEntropyLoss later
        return x
```

1. Dataset Loading:

-   The MNIST dataset, a collection of handwritten digits, is loaded using PyTorch's torchvision.datasets.MNIST class. This process involves specifying parameters such as root for the storage directory, train to toggle between training and testing sets, transform to apply transformations like converting images to tensor format, and download to download the data if it's not already present locally.

```python mnist_classifier.ipynb
import numpy as np
import os
import torch
from torchvision.datasets import mnist
from torch.utils.data import DataLoader
from torchvision.transforms import ToTensor

device = 'cuda' if torch.cuda.is_available() else 'cpu'
batch_size = 256
train_dataset = mnist.MNIST(root='./train', train=True, transform=ToTensor(), download=True)
test_dataset = mnist.MNIST(root='./test', train=False, transform=ToTensor(), download=True)
train_loader = DataLoader(train_dataset, batch_size=batch_size)
test_loader = DataLoader(test_dataset, batch_size=batch_size)
```

2. Normalization Function:

-   The normalize_img function is a crucial part of the data preparation. 
-   It rounds the pixel values of the images to 0 or 1. This step mimics th binary nature of the input data expected from a drawing interface, where the pixels are either filled or not. We will build this out in part 4 of the tutorial where we focus on the frontend.

```python mnist_classifier.ipynb
def normalize_img(image, label):
  return torch.round(image), label
```

3. Data Pipeline for Training Set:
-   The training dataset undergoes normalization using the DataLoader class, which combines the dataset and a sampler, providing an iterable over the dataset.
-   Data is batched with a size of 256, meaning 256 images are processed in each iteration of training.
-   No explicit shuffling or prefetching is used here, but these could be incorporated for enhanced performance.

4. Data Pipeline for Testing Set:
-   The testing dataset is similarly normalized and batched using the DataLoader.
-   The batching process for the test dataset mirrors the training set, facilitating a consistent evaluation process.

### Training

1. Model Configuration:
-   The model is moved to the appropriate device (GPU if available, otherwise CPU) using PyTorch’s `.to(device)` method.
-   An instance of the Adam optimizer is created with the model's parameters. Adam is chosen for its efficiency in handling sparse gradients and adaptive learning rate capabilities.
-   The loss function is defined as CrossEntropyLoss, a standard choice for classification tasks with multiple classes.

2. Model Training:
-   The training loop runs for a predefined number of epochs (25 in this case). Each epoch consists of a forward pass where predictions are generated, a loss calculation, and a   backward pass for gradients computation and parameters update.
-   The accuracy of the model is evaluated at the end of each epoch on the test dataset. This provides insight into the model's generalization capability on unseen data.

```python mnist_classifier.ipynb
model = LeNet().to(device)
adam = Adam(model.parameters())  # Using Adam optimizer
loss_fn = CrossEntropyLoss()
all_epoch = 25
prev_acc = 0
for current_epoch in range(all_epoch):
    model.train()
    for idx, (train_x, train_label) in enumerate(train_loader):
        train_x = train_x.to(device)
        # normalize the image to 0 or 1 to reflect the inputs from the drawing board
        train_x = train_x.round()
        train_label = train_label.to(device)
        adam.zero_grad()  # Use adam optimizer
        predict_y = model(train_x.float())
        loss = loss_fn(predict_y, train_label.long())
        loss.backward()
        adam.step()  # Use adam optimizer
    all_correct_num = 0
    all_sample_num = 0
    model.eval()

    for idx, (test_x, test_label) in enumerate(test_loader):
        test_x = test_x.to(device)
         # normalize the image to 0 or 1 to reflect the inputs from the drawing board
        test_x = test_x.round()
        test_label = test_label.to(device)
        predict_y = model(test_x.float()).detach()
        predict_y = torch.argmax(predict_y, dim=-1)
        current_correct_num = predict_y == test_label
        all_correct_num += np.sum(current_correct_num.to('cpu').numpy(), axis=-1)
        all_sample_num += current_correct_num.shape[0]
    acc = all_correct_num / all_sample_num
    print('test accuracy: {:.3f}'.format(acc), flush=True)
    if not os.path.isdir("models"):
        os.mkdir("models")
    torch.save(model, 'models/mnist_{:.3f}.pkl'.format(acc))
    prev_acc = acc
```

### Exporting to ONNX

The trained PyTorch model is then converted to the ONNX (Open Neural Network Exchange) format using torch.onnx.export, which facilitates model interoperability and makes it compatible with various platforms and tools. The conversion process involves specifying the input shape and the ONNX opset version (always set to 12). Alongside, an input data file (input.json) is created by extracting a sample dataset from the training dataset and serializing it into JSON format. This data is used later for EZKL hub deployment.

```python mnist_classifier.ipynb
import torch
import json
import os

model_path = os.path.join('network_lenet.onnx')

model.eval()  # Set the model to evaluation mode

# # Fetch a single data point from the train_dataset
# # Ensure train_dataset is already loaded and accessible
train_data_point, _ = next(iter(train_dataset))
train_data_point = train_data_point.unsqueeze(0)  # Add a batch dimension

# Verify the device (CPU or CUDA) and transfer the data point to the same device as the model
device = 'cuda' if torch.cuda.is_available() else 'cpu'
train_data_point = train_data_point.to(device)

# # Export the model to ONNX format
torch.onnx.export(model, train_data_point, model_path, export_params=True, opset_version=12, do_constant_folding=True, input_names=['input_0'], output_names=['output'])

# Convert the tensor to numpy array and reshape it for JSON serialization
x = train_data_point.cpu().detach().numpy().reshape([-1]).tolist()
data = {'input_data': [x]}
with open('input.json', 'w') as f:
    json.dump(data, f)

print(f"Model exported to {model_path} and input data saved to input.json")
```

### Deploying to the EZKL Hub

If you have been following along with the `mnist_classifier` notebook, you should be on cell number [7](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#scrollTo=dS-yXte30rZ3). There is another version of this notebook on the main EZKL repo where we perform the setup and proving all locally [here](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#scrollTo=dS-yXte30rZ3). Comparing this version of the notebook to the one on the e2e-mnist repo, you may notice the cells are identical up until cell 7. After this point we generate a lot of zk specific artifacts (namely a settings file, a compiled circuit, a proving key, a verification key, a witness, a proof and solidity verifier) whereas with the other notebook we only have one more cell that calls into the hub. This is a lot of artifacts to manage and takes a lot of compute resources to generate. Moreover, once you have the artifacts necessary to generate proofs, you then have to set up a proving server if you want to generate proofs on behalf of your users.

To make this process easier, we have created a backend proving service called [EZKL Hub](https://app.ezkl.xyz/) that generates and manages these artifacts and makes serving proofs as easy as calling a graph QL endpoint. To deploy your model to the hub, you will need to sign into EZKL Hub using your Github. 

Once you approve EZKL Hub access to your Github openid profile, modify cell [7](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/mnist_classifier.ipynb#scrollTo=dS-yXte30rZ3) by adding your Github username to the `test_hub_name` variable.

Specifically we:

1. Initialize the EZKL run args: These are the arguments that determine the visibility of the inputs, parameters and outputs of the model. We set the input visibility to private, the parameter visibility to fixed and the output visibility to public. This means that the inputs to the model will be private to the verifier, the parameters will be set as fixed columns in the circuit (meaning that once the circuit has been created they are hardcoded into the cicuit as fixed columns, so they can't be updated later) and the outputs will be visible to the verifier.

```python mnist_classifier.ipynb
import ezkl

run_args = ezkl.PyRunArgs()
run_args.input_visibility = "private"
run_args.param_visibility = "fixed"
run_args.output_visibility = "public"
run_args.num_inner_cols = 2
run_args.variables = [("batch_size", 1)]

```

2. Gather Calibration Data: We gather a set of data points from the training set to use for calibration. Calibrating the circuit with sample inputs like this ensures that the lookup tables within our circuit are robust to outliers and therefore not likely to fall out of range of the [lookups](https://zcash.github.io/halo2/design/proving-system/lookup.html) (we represent non-linearities as lookup tables).  

```python mnist_classifier.ipynb
# Capture set of data points
num_data_points = 8

# Fetch 30 data points from the train_dataset
data_points = []
for i, (data_point, _) in enumerate(train_dataset):
    if i >= num_data_points:
        break
    data_points.append(data_point)

# Stack the data points to create a batch
train_data_batch = torch.stack(data_points)

# Add a batch dimension if not already present
if train_data_batch.dim() == 3:
    train_data_batch = train_data_batch.unsqueeze(0)

x = train_data_batch.cpu().detach().numpy().reshape([-1]).tolist()

data = dict(input_data = [x])

cal_path = os.path.join('cal_data.json')
```

3. Deploy the Model to the Hub: We deploy the model to the hub using the `create_hub_artifact` function. This function takes in the model path, the data path, the name of the model, the organization id, the calibration data path, the target directory and the run args. The model path is the path to the ONNX model we exported in the previous section. The input data is used to ensure that all of the constraints of the model hold. We set the calibration target to `resources` so that circuit settings favor a more cost efficient proving. You can also set the target to `accuracy` if you want to favor a more accurate circuit (accurate in the sense that the outputs are closer to the outputs of the canonical onnx model) at the expense of increased proving time.
    
```python mnist_classifier.ipynb
test_hub_name = "samtvlabs" # we've set this up for you, but you can create your own hub name and use that instead

organization = ezkl.get_hub_credentials(test_hub_name)['organizations'][0]

print("organization: " + str(organization))

name = "example_name" # set whatever name you want here

deployed_model = ezkl.create_hub_artifact(model_path, data_path, name, organization['id'], cal_input=cal_path, target="resources", py_run_args=run_args)

print("deployed model: " + str(deployed_model))

# Loop every 5 seconds until status is not pending
status = "PENDING"
while status == "PENDING":
    time.sleep(5)
    get_model = ezkl.get_hub_artifact(deployed_model['id'])
    status = get_model['status']
print("status: " + status)
```

4. Test proof: Now that we have generated our artifacts, we can generate proofs against them on the hub to ensure that the our model was deployed properly. In part 4 of this tutorial when we start building out our client we will be calling this same EZKL hub endpoint to generate proofs on behalf of our users.

```python mnist_classifier.ipynb
# GENERATE PROOF
proof_id = ezkl.prove_hub(deployed_model['id'], data_path)
print("proof id: " + str(proof_id))

# Loop every 5 seconds until status is not pending
status = "PENDING"
while status == "PENDING":
    time.sleep(5)
    get_proof = ezkl.get_hub_proof(proof_id['id'])
    status = get_proof['status']

proof = ezkl.get_hub_proof(proof_id['id'])

print("proof: " + str(proof))
```
