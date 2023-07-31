---
icon: code-square
order: 93
---
![](../assets/cli.png) 
The `ezkl` cli provides a simple interface to load `.onnx` files, which represent graphs of operations (such as neural networks), convert them, and run a proof.

## CLI tutorial 👾

You can easily create an `.onnx` file using `pytorch`. For samples of Onnx files see [here](https://github.com/onnx/models). To see how to generate Onnx files using python, check out <a href="https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/" target="_blank"> the notebooks.</a>. You'll also need an `input.json` file with sample inputs and outputs of your model.

Sample onnx files are also available in <a href="https://github.com/zkonduit/ezkl/blob/main/examples/onnx/" target="_blank"> the repo </a>.

#### Initializing the project
To generate a proof on one of the examples, first install `ezkl` 
[!ref](/getting_started)

Put a model file (`network.onnx`) and input file (`input.json`) into your working directory, e.g. with something like:
```bash
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/network.onnx ./
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/input.json ./

```
To display `ezkl`'s understanding of the model, run:

```bash
ezkl table -M network.onnx
```

#### Setting circuit parameters
Our circuit is configured with the `settings.json` file. This is created with the `gen-settings` command. 
```bash
ezkl gen-settings -M network.onnx
```
This will produce a `settings.json` file you can use for your circuit. However, you can fine-tune your circuit to optimize for accuracy or CPU/memory usage with the `calibrate-settings` command:
```bash
ezkl calibrate-settings -M network.onnx -D input.json --target resources
```
In this example, we set the `--target` to **"resources"** so that we can optimize for CPU and memory usage. The other option is **"accuracy"**, which optimizes for accuracy given the fixed point representation of the input model. Our circuit parameters are generated, then saved to `settings.json`. You can pass a `--settings-path` to read from an existing settings file, and only modify the parts changed by calibration (e.g. leaving visibility or tolerance unchanged). You can customize this file and even change the way it is generated. Learn more about `gen-settings` and `calibrate-settings` in the [Commands](https://docs.ezkl.xyz/about_ezkl/commands/) section.

Download the appropriate SRS:
```bash
ezkl get-srs -S settings.json
```


#### Compiling the model
From the onnx file, we will create a `.ezkl` file that uses the settings to convert the onnx model to a format ready for proving.

```bash
ezkl compile-model -M network.onnx -S settings.json --compiled-model network.ezkl
```


#### Creating the circuit
Now, we use `setup` to create a proving and verifying key for our circuit, using the SRS, our circuit settings, and the .onnx file. 

```bash
ezkl setup -M network.ezkl --srs-path=kzg.srs --vk-path=vk.key --pk-path=pk.key --settings-path=settings.json
```
This creates the verification key, proving key, and circuit settings in the locations you specify. 

> Note: You can view the options associated to a subcommand such as `setup` by typing `ezkl setup` with no parameters. If you provide some but not all required parameters, `ezkl` will tell you what else it needs.

#### Making a proof
First we generate a witness file.

```bash
ezkl gen-witness -D input.json -M network.ezkl -S settings.json
```

Next we will generate a proof that the model was correctly run on private inputs (this is the default setting). It then outputs the resulting proof at the path specfifed by `--proof-path`.

```bash
ezkl prove -M network.ezkl --witness witness.json --pk-path=pk.key --proof-path=model.proof --srs-path=kzg.srs --settings-path=settings.json
```

#### Verification
We can then verify our generated proof with the `verify` command:
```bash
ezkl verify --proof-path=model.proof --settings-path=settings.json --vk-path=vk.key --srs-path=kzg.srs
```

#### All together
```bash
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/network.onnx ./
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/input.json ./
ezkl gen-settings -M network.onnx
ezkl calibrate-settings -M network.onnx -D input.json --target resources
ezkl get-srs -S settings.json
ezkl compile-model -M network.onnx -S settings.json --compiled-model network.ezkl
ezkl setup -M network.ezkl --srs-path=kzg.srs --vk-path=vk.key --pk-path=pk.key --settings-path=settings.json
ezkl gen-witness -D input.json -M network.ezkl -S settings.json
ezkl prove -M network.ezkl --witness witness.json --pk-path=pk.key --proof-path=model.proof --srs-path=kzg.srs --settings-path=settings.json
ezkl verify --proof-path=model.proof --settings-path=settings.json --vk-path=vk.key --srs-path=kzg.srs
```



