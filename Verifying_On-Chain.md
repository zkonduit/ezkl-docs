---
icon: workflow
order: 7
---
#### Verifying with the EVM ◊

To verify on-chain, generate a verifier smart contract after performing setup.

You can use the example from Commands, or create it by copying over a network and input file (assuming the ezkl repo is in your home directory):

```bash
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/network.onnx ./
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/input.json ./
```

then create the setup

```bash
ezkl gen-settings -M network.onnx
ezkl calibrate-settings -M network.onnx -D input.json --target resources
ezkl get-srs -S settings.json
ezkl compile-circuit -M network.onnx -S settings.json --compiled-circuit network.ezkl
ezkl setup -M network.ezkl --srs-path=kzg.srs
```

Now we use this setup to create an EVM verifier, which would be deployed on-chain.

```bash
# gen evm verifier
ezkl create-evm-verifier --srs-path=kzg.srs --vk-path vk.key --sol-code-path verif.sol --settings-path=settings.json
```

```bash
ezkl gen-witness -D input.json -M network.ezkl
ezkl prove --witness witness.json -M network.ezkl --proof-path model.pf --pk-path pk.key --srs-path=kzg.srs
```

```bash
# install anvil if you haven't already
cargo install --git https://github.com/foundry-rs/foundry --profile local --locked anvil
```

```bash
# spin up a local EVM through anvil in a separate terminal 
anvil -p 3030
```

```bash
# deploy evm verifier
ezkl deploy-evm-verifier --addr-path=addr.txt --rpc-url=http://127.0.0.1:3030 --sol-code-path verif.sol 
```

```bash
# verify (EVM), make sure to copy the address stored in addr.txt and paste it into the addr param
ezkl verify-evm --proof-path model.pf --addr=$(cat addr.txt) --rpc-url=http://127.0.0.1:3030
```

Note that the `.sol` file above can be deployed and composed with other Solidity contracts, via a `verify()` function.

#### Aggregation

> Note: this is an advanced technique, very resource-intensive, and chances are you don't want to use this; if you are having trouble with proof size, first try asking about your problem in Telegram or Discord.

The above pipeline can also be run using proof aggregation to reduce the final proof size and the size and execution cost of the on-chain verifier. A sample pipeline for doing so would be as follows.

Grab a smaller model.

```bash
cp ~/ezkl/examples/onnx/1l_relu/network.onnx ./
cp ~/ezkl/examples/onnx/1l_relu/input.json ./
```

```bash
# Generate a new SRS. We use 20 since aggregation requires larger circuits (more commonly 23+).
ezkl gen-srs --logrows 20 --srs-path=20.srs
```

```bash
# Create new circuit parameters
ezkl gen-settings -M network.onnx
ezkl calibrate-settings -M network.onnx -D input.json --target resources
```

Set up the first proof.

```bash
# Set up a new circuit
ezkl compile-circuit -M network.onnx -S settings.json --compiled-circuit network.ezkl
ezkl setup  -M network.ezkl --srs-path=20.srs --vk-path=vk.key --pk-path=pk.key 
```

```bash
# Single proof -> single proof we are going to feed into aggregation circuit. (Mock)-verifies + verifies natively as sanity check
ezkl gen-witness -D input.json -M network.ezkl
ezkl prove --proof-type=for-aggr -W witness.json -M network.ezkl --proof-path first.pf --srs-path=20.srs  --pk-path=pk.key
```

Setup the aggregate proof.

```bash
ezkl setup-aggregate --sample-snarks first.pf --srs-path 20.srs --logrows 20
```

```bash
# Aggregate -> generates aggregate proof
ezkl aggregate --logrows=20 --aggregation-snarks=first.pf --srs-path=20.srs --pk-path pk_aggr.key 
```

```bash
# Generate aggregate evm verifier
ezkl create-evm-verifier-aggr --sol-code-path verif.sol --srs-path=20.srs --vk-path vk_aggr.key --aggregation-settings=settings.json
```

```bash
# Spin up a local EVM through anvil 
anvil -p 3030
```

```bash
# deploy evm verifier
ezkl deploy-evm-verifier --addr-path=addr.txt --rpc-url=http://127.0.0.1:3030 --sol-code-path verif.sol 
```

```bash
# verify (EVM), make sure to copy the address stored in addr.txt and paste it into the addr param
ezkl verify-evm --proof-path proof_aggr.proof --addr=$(cat addr.txt) --rpc-url=http://127.0.0.1:3030
```

Also note that this may require a local [solc](https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html) installation. You can follow the SolidityLang instructions linked above, or you can use [svm-rs](https://github.com/alloy-rs/svm-rs) to install solc. Here's how:

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
