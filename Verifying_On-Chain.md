---
icon: workflow
order: 7
---
#### verifying with the EVM ◊

Verification can also be run with an EVM verifier. This can be done by generating a verifier smart contract after performing setup.

You can use the example from Commands, or create it by copying over a network and input file (assuming the ezkl repo is in your home directory):
```bash
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/network.onnx ./
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/input.json ./
```
then create the setup
```bash
ezkl gen-srs --logrows 15 --srs-path=15.srs
ezkl gen-settings -M network.onnx
ezkl calibrate-settings -M network.onnx -D input.json --target resources
ezkl setup -M network.onnx --srs-path=15.srs --settings-path=settings.json
```

Now we use the setup to create an EVM verifier, which would be deployed on-chain. 

```bash
# gen evm verifier
ezkl create-evm-verifier --deployment-code-path verif.code --srs-path=15.srs --vk-path vk.key --sol-code-path verif.sol --settings-path=settings.json
```

```bash
ezkl prove --transcript=evm -D input.json -M network.onnx --proof-path model.pf --pk-path pk.key --srs-path=15.srs --settings-path=settings.json 
```

```bash
# Verify (EVM)
ezkl verify-evm --proof-path model.pf --deployment-code-path verif.code
```

Note that the `.sol` file above can be deployed and composed with other Solidity contracts, via a `verify()` function. Please read [this document](https://hackmd.io/QOHOPeryRsOraO7FUnG-tg) for more information about the interface of the contract, how to obtain the data needed for its function parameters, and its limitations.

The above pipeline can also be run using proof aggregation to reduce the final proof size and the size and execution cost of the on-chain verifier. A sample pipeline for doing so would be:

```bash
# Generate a new SRS. We use 20 since aggregation requires larger circuits (more commonly 23+).
ezkl gen-srs --logrows 20 --srs-path=20.srs
```

```bash
# Create new circuit parameters
ezkl gen-circuit-params --calibration-target resources --model examples/onnx/1l_relu/network.onnx --settings-path circuit.json
```

```bash
# Set up a new circuit
ezkl setup  -M examples/onnx/1l_relu/network.onnx --srs-path=20.srs --vk-path=vk.key --pk-path=pk.key --settings-path=circuit.json
```

```bash
# Single proof -> single proof we are going to feed into aggregation circuit. (Mock)-verifies + verifies natively as sanity check
ezkl prove --transcript=poseidon --strategy=accum -D ./examples/onnx/1l_relu/input.json -M ./examples/onnx/1l_relu/network.onnx --proof-path 1l_relu.pf --srs-path=20.srs  --pk-path=pk.key --settings-path=circuit.json
```

```bash
# Aggregate -> generates aggregate proof and also (mock)-verifies + verifies natively as sanity check
ezkl aggregate --logrows=20 --aggregation-snarks=1l_relu.pf --aggregation-vk-paths vk.key --vk-path aggr_1l_relu.vk --proof-path aggr_1l_relu.pf --srs-path=20.srs --settings-paths=circuit.json
```

```bash
# Generate verifier code -> create the EVM verifier code
ezkl create-evm-verifier-aggr --deployment-code-path aggr_1l_relu.code --srs-path=20.srs --vk-path aggr_1l_relu.vk
```

```bash
# Verify (EVM) ->
ezkl verify-aggr --logrows=20 --proof-path aggr_1l_relu.pf --srs-path=20.srs --vk-path aggr_1l_relu.vk
```

Also note that this may require a local [solc](https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html) installation, and that aggregated proof verification in Solidity is not currently supported. You can follow the SolidityLang instructions linked above, or you can use [svm-rs](https://github.com/alloy-rs/svm-rs) to install solc. Here's how:

Install svm-rs:
```bash
cargo install svm-rs
```

Install a recent Solidity version (we use 0.8.20 in our implementation):
```bash
svm install 0.8.20
```

Verify your Solidity version:
```bash
solc --version
```

