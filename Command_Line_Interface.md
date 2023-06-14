---
icon: code-square
order: 8
---
![](../assets/cli.png) 
The `ezkl` cli provides a simple interface to load `.onnx` files, which represent graphs of operations (such as neural networks), convert them into a Halo2 circuit, then run a proof.

## CLI tutorial 👾

You can easily create an `.onnx` file using `pytorch`. For samples of Onnx files see [here](https://github.com/onnx/models). For a tutorial on how to quickly generate Onnx files using python, check out [pyezkl](https://github.com/zkonduit/pyezkl). You'll also need an `input.json` file with sample inputs and outputs of your model (Note: input shape is no longer needed since this is now inferred by the library).

Sample onnx files are also available in `./examples/onnx`.
#### Initializing the project
To generate a proof on one of the examples, first install `ezkl` 
[!ref](/getting_started)
then generate a structured reference string (SRS):
```bash
ezkl gen-srs --logrows 17 --params-path=kzg.params
```
Note that this SRS is for testing purposes only. 

#### Setting circuit parameters
We then set up the circuit parameters by calibrating our model with `gen-circuit-params`. This command generates a `.json` file of our circuit parameters for future use (replace `network.onnx` with the relative path of one of the example's `.onnx` files):
```bash
ezkl gen-circuit-params --calibration-target resources --model network.onnx --circuit-params-path circuit.json
```
In this example, we will set the `calibration-target` to **"resources"** so that we can optimize for CPU and memory usage. The other option is **"accuracy"**, which optimizes for accuracy given the fixed point representation of the input model. Our circuit parameters are generated, then saved to `circuit.json`. You can customize this file and even change the way it's generated. Learn more about `gen-circuit-params` in the [Commands](https://docs.ezkl.xyz/about_ezkl/commands/) section. 

#### Creating the circuit
Now, we use `setup` to create a proving and verifying key for our circuit, using the SRS, our circuit params, and the .onnx file. 

```bash
ezkl setup -M network.onnx --params-path=kzg.params --vk-path=vk.key --pk-path=pk.key --circuit-params-path=circuit.json
```
This creates the verification key, proving key, and circuit params in the locations you specify. There are a lot of options during setup, which you can view by typing
```bash
ezkl setup 
```

#### Making a proof
Next we will generate a proof that the model was correctly run on private inputs (this is the default setting). It then outputs the resulting proof at the path specfifed by `--proof-path`.

```bash
ezkl prove -M network.onnx -D input.json --pk-path=pk.key --proof-path=model.proof --params-path=kzg.params --circuit-params-path=circuit.json
```

#### Verification
We can then verify our generated proof with the `verify` command:
```bash
ezkl verify --proof-path=model.proof --circuit-params-path=circuit.json --vk-path=vk.key --params-path=kzg.params
```

#### Visualizing our model
To display a table of the loaded onnx nodes, their associated parameters, set `RUST_LOG=DEBUG` or run:

```bash
ezkl table -M ./examples/onnx/1l_relu/network.onnx
```

## Using a pre-generated SRS

Note that you can use pre-generated KZG SRS. These SRS can be converted to a format that is ingestable by the `pse/halo2` prover `ezkl` uses by leveraging [han0110/halo2-kzg-srs](https://github.com/han0110/halo2-kzg-srs). This repo also contains pre-converted SRS from large projects such as Hermez and the [perpetual powers of tau repo](https://github.com/privacy-scaling-explorations/perpetualpowersoftau). Simply download the pre-converted file locally and point `--params-path` to the file.

> Note: Ensure you download the files in raw format. As this will be more performant and is the serialization format `ezkl` assumes.

## General usage 🔧
This is a comprehensive list of the commands and flags you can use with `ezkl`. Learn more about the Commands here:
[!ref](/About_EZKL/Commands)

And the flags (RunArgs) here:
[!ref](/About_EZKL/RunArgs)

```bash
Usage: ezkl [OPTIONS] <COMMAND>

Commands:
  table                     Loads model and prints model table
  forward                   Runs a vanilla forward pass, produces a quantized output, and saves it to a .json file
  gen-circuit-params        Calibrates the proving hyperparameters, produces a quantized output from those hyperparameters, and saves it to a .json file. The circuit parameters are also saved to a file
  gen-srs                   Generates a dummy SRS
  mock                      Loads model and input and runs mock prover (for testing)
  aggregate                 Aggregates proofs :)
  setup                     Creates pk and vk and circuit params
  fuzz                      Fuzzes the proof pipeline with random inputs, random parameters, and random keys
  prove                     Loads model, data, and creates proof
  create-evm-verifier       Creates an EVM verifier for a single proof
  create-evm-verifier-aggr  Creates an EVM verifier for an aggregate proof
  verify                    Verifies a proof, returning accept or reject
  verify-aggr               Verifies an aggregate proof, returning accept or reject
  verify-evm                Verifies a proof using a local EVM executor, returning accept or reject
  print-proof-hex           Print the proof in hexadecimal
  help                      Print this message or the help of the given subcommand(s)

Options:
  -D, --data <DATA>
          The path to the .json data file
  -M, --model <MODEL>
          The path to the .onnx model file
  -T, --tolerance <TOLERANCE>
          The tolerance for error on model outputs [default: 0]
  -S, --scale <SCALE>
          The denominator in the fixed point representation used when quantizing [default: 7]
  -B, --bits <BITS>
          The number of bits used in lookup tables [default: 16]
  -K, --logrows <LOGROWS>
          The log_2 number of rows [default: 17]
      --batch-size <BATCH_SIZE>
          The number of batches to split the input data into [default: 1]
      --input-visibility <INPUT_VISIBILITY>
          Flags whether inputs are public, private, hashed [default: private]
      --output-visibility <OUTPUT_VISIBILITY>
          Flags whether outputs are public, private, hashed [default: public]
      --param-visibility <PARAM_VISIBILITY>
          Flags whether params are public, private, hashed [default: private]
      --pack-base <PACK_BASE>
          Base used to pack the public-inputs to the circuit. (value > 1) to pack instances as a single int. Useful when verifying on the EVM. Note that this will often break for very long inputs. Use with caution, still experimental [default: 1]
      --allocated-constraints <ALLOCATED_CONSTRAINTS>
          the number of constraints the circuit might use. If not specified, this will be calculated using a 'dummy layout' pass
      --circuit-params-path <CIRCUIT_PARAMS_PATH>
          optional circuit params path (overrides any run args set)
  -h, --help
          Print help
```

`prove` and `mock` both require `-D` and `-M` parameters. 

```bash
Usage: ezkl mock [OPTIONS]

Options:
  -D, --data <DATA>    The path to the .json data file [default: ]
  -M, --model <MODEL>  The path to the .onnx model file [default: ]

```

The `.onnx` file can be generated using pytorch or tensorflow. 

For examples of such files see `examples/onnx_models`.
