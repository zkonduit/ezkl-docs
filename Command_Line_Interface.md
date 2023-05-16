---
icon: code-square
order: 8
---

The `ezkl` cli provides a simple interface to load `.onnx` files, which represent graphs of operations (such as neural networks), convert them into a Halo2 circuit, then run a proof.

### python and cli tutorial 🐍

You can easily create an `.onnx` file using `pytorch`. For samples of Onnx files see [here](https://github.com/onnx/models). For a tutorial on how to quickly generate Onnx files using python, check out [pyezkl](https://github.com/zkonduit/pyezkl).

Sample onnx files are also available in `./examples/onnx`. To generate a proof on one of the examples, first build `ezkl` (`cargo build --release`) and add it to your favourite `PATH` variables, then generate a structured reference string (SRS):
```bash
ezkl gen-srs --logrows 17 --params-path=kzg.params
```
We then set up the circuit to create a proving and verifying key for our circuit. You must provide the input.json and network.onnx files. 

```bash
ezkl setup -D input.json -M network.onnx --params-path=kzg.params --vk-path=vk.key --pk-path=pk.key --circuit-params-path=circuit.params
```

This command generates a proof that the model was correctly run on private inputs (this is the default setting). It then outputs the resulting proof at the path specfifed by `--proof-path`, parameters that can be used for subsequent verification at `--params-path` and the verifier key at `--vk-path`:

```bash
ezkl prove -M network.onnx -D input.json --pk-path=pk.key --proof-path=model.proof --params-path=kzg.params --circuit-params-path=circuit.params
```

We can then verify our generated proof with the `verify` command:
```bash
ezkl verify --proof-path=model.proof --circuit-params-path=circuit.params --vk-path=vk.key --params-path=kzg.params
```

To display a table of the loaded onnx nodes, their associated parameters, set `RUST_LOG=DEBUG` or run:

```bash
cargo run --release --bin ezkl -- table -M ./examples/onnx/1l_relu/network.onnx

```


### using pre-generated SRS

Note that you can use pre-generated KZG SRS. These SRS can be converted to a format that is ingestable by the `pse/halo2` prover `ezkl` uses by leveraging [han0110/halo2-kzg-srs](https://github.com/han0110/halo2-kzg-srs). This repo also contains pre-converted SRS from large projects such as Hermez and the [perpetual powers of tau repo](https://github.com/privacy-scaling-explorations/perpetualpowersoftau). Simply download the pre-converted file locally and point `--params-path` to the file.

> Note: Ensure you download the files in raw format. As this will be more performant and is the serialization format `ezkl` assumes.



### general usage 🔧

> Note: to get the full suite of cli capabilities you'll need to compile `ezkl` with the `render` feature (`cargo build --features render --bin ezkl`). This enables the `render-circuit` command which can create `.png` representations of the compiled circuits. You'll also need to install the `libexpat1-dev` and `libfreetype6-dev` libraries on Debian systems (there are equivalents for MacOS as well).

```bash
Usage: ezkl [OPTIONS] <COMMAND>

Commands:
  table                     Loads model and prints model table
  forward                   Runs a vanilla forward pass, produces a quantized output, and saves it to a .json file
  gen-srs                   Generates a dummy SRS
  mock                      Loads model and input and runs mock prover (for testing)
  aggregate                 Aggregates proofs :)
  prove                     Loads model and data, prepares vk and pk, and creates proof
  create-evm-verifier       Creates an EVM verifier for a single proof
  create-evm-verifier-aggr  Creates an EVM verifier for an aggregate proof
  deploy-verifier-evm       Deploys an EVM verifier
  send-proof-evm            Send a proof to be verified to an already deployed verifier
  verify                    Verifies a proof, returning accept or reject
  verify-aggr               Verifies an aggregate proof, returning accept or reject
  verify-evm                Verifies a proof using a local EVM executor, returning accept or reject
  print-proof-hex           Print the proof in hexadecimal
  help                      Print this message or the help of the given subcommand(s)

Options:
  -T, --tolerance <TOLERANCE>
          The tolerance for error on model outputs [default: 0]
  -S, --scale <SCALE>
          The denominator in the fixed point representation used when quantizing [default: 7]
  -B, --bits <BITS>
          The number of bits used in lookup tables [default: 16]
  -K, --logrows <LOGROWS>
          The log_2 number of rows
      --public-inputs
          Flags whether inputs are public
      --public-outputs <PUBLIC_OUTPUTS>
          Flags whether outputs are public [default: true] [possible values: true, false]
      --public-params
          Flags whether params are public
      --pack-base <PACK_BASE>
          Base used to pack the public-inputs to the circuit. (value > 1) to pack instances as a single int. Useful when verifying on the EVM. Note that this will often break for very long inputs. Use with caution, still experimental [default: 1]
      --single-lookup <SINGLE_LOOKUP>
          use a single argument for all lookups [default: true] [possible values: true, false]
      --check-mode <CHECK_MODE>
          use a single argument for all lookups [default: safe]
  -h, --help
          Print help
  -V, --version
          Print version
```

`bits`, `scale`, `tolerance`, and `logrows` have default values. You can use tolerance to express a tolerance to a certain amount of quantization error on the output eg. if set to 2 the circuit will verify even if the generated output deviates by an absolute value of 2 on any dimension from the expected output. `prove` and `mock`, all require `-D` and `-M` parameters, which if not provided, the cli will query the user to manually enter the path(s).

```bash

Usage: ezkl mock [OPTIONS]

Options:
  -D, --data <DATA>    The path to the .json data file [default: ]
  -M, --model <MODEL>  The path to the .onnx model file [default: ]

```

The `.onnx` file can be generated using pytorch or tensorflow. The data json file is structured as follows:

```javascript
{
    "input_data": [[1.0, 22.2, 0.12 ...]], // 2D arrays of floats which represents the (private) inputs we run the proof on
    "input_shapes": [[3, 3, ...]], // 2D array of integers which represents the shapes of model inputs (excluding batch size)
    "output_data": [[1.0, 5.0, 6.3 ...]], // 2D arrays of floats which represents the model outputs we want to constrain against (if any)
}
```

For examples of such files see `examples/onnx_models`.

To run a simple example using the cli see **python and cli tutorial** above.

If the above commands get too heavy and it becomes difficult to track parameters across commands; `ezkl` also supports loading global arguments (those specified before a subcommand) from a `.json` file. This can be done using the `RUNARGS` environment variable. For instance:

```bash
RUNARGS=/path/to/args.json ezkl subcommand --subcommand-params...
```
For an example of such a file see `examples/default_run_args.json`:
```json
{
    "tolerance": 0,
    "scale": 7,
    "bits": 11,
    "logrows": 17,
    "public_inputs": false,
    "public_outputs": true,
    "public_params": false,
}
```

Note that command-wide arguments can be specified using the `EZKLCONF` environment variable; which supercedes `RUNARGS` in priority !
This json includes both global level arguments _and_ subcommand specific arguments. Usage is thus as such:
```bash
EZKLCONF=/path/to/fullconfig.json ezkl
```

