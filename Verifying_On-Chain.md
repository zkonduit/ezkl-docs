---
icon: workflow
order: 7
---
#### verifying with the EVM ◊

Note that the above prove and verify stats can also be run with an EVM verifier. This can be done by generating a verifier smart contract after generating the proof

```bash
# gen proof
ezkl prove --transcript=evm -D ./examples/onnx/1l_relu/input.json -M ./examples/onnx/1l_relu/network.onnx --proof-path 1l_relu.pf --pk-path pk.key --params-path=kzg.params --circuit-params-path=circuit.params 
```
```bash
# gen evm verifier
ezkl create-evm-verifier --deployment-code-path 1l_relu.code --params-path=kzg.params --vk-path vk.key --sol-code-path 1l_relu.sol --circuit-params-path=circuit.params
```
```bash
# Verify (EVM)
ezkl verify-evm --proof-path 1l_relu.pf --deployment-code-path 1l_relu.code
```

Note that the `.sol` file above can be deployed and composed with other Solidity contracts, via a `verify()` function. Please read [this document](https://hackmd.io/QOHOPeryRsOraO7FUnG-tg) for more information about the interface of the contract, how to obtain the data needed for its function parameters, and its limitations.

The above pipeline can also be run using [proof aggregation](https://ethresear.ch/t/leveraging-snark-proof-aggregation-to-achieve-large-scale-pbft-based-consensus/11588) to reduce proof size and verifying times, so as to be more suitable for EVM deployment. A sample pipeline for doing so would be:

```bash
# Generate a new SRS. We use 20 since aggregation requires larger circuits.
ezkl gen-srs --logrows 20 --params-path=kzg.params
```

```bash
# Set up a new circuit
ezkl setup -D examples/onnx/1l_relu/input.json -M examples/onnx/1l_relu/network.onnx --params-path=kzg.params --vk-path=vk.key --pk-path=pk.key --circuit-params-path=circuit.params
```

```bash
# Single proof -> single proof we are going to feed into aggregation circuit. (Mock)-verifies + verifies natively as sanity check
ezkl prove --transcript=poseidon --strategy=accum -D ./examples/onnx/1l_relu/input.json -M ./examples/onnx/1l_relu/network.onnx --proof-path 1l_relu.pf --params-path=kzg.params  --pk-path=pk.key --circuit-params-path=circuit.params
```

```bash
# Aggregate -> generates aggregate proof and also (mock)-verifies + verifies natively as sanity check
ezkl aggregate --logrows=17 --aggregation-snarks=1l_relu.pf --aggregation-vk-paths 1l_relu.vk --vk-path aggr_1l_relu.vk --proof-path aggr_1l_relu.pf --params-path=kzg.params --circuit-params-paths=circuit.params
```

```bash
# Generate verifier code -> create the EVM verifier code
ezkl create-evm-verifier-aggr --deployment-code-path aggr_1l_relu.code --params-path=kzg.params --vk-path aggr_1l_relu.vk
```

```bash
# Verify (EVM) ->
ezkl verify-aggr --logrows=20 --proof-path aggr_1l_relu.pf --params-path=kzg.params --vk-path aggr_1l_relu.vk
```

Also note that this may require a local [solc](https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html) installation, and that aggregated proof verification in Solidity is not currently supported.

You can also send proofs to be verified on deployed contracts using `send-proof`:

```bash
Send a proof to be verified to an already deployed verifier

Usage: ezkl send-proof-evm --secret <SECRET> --rpc-url <RPC_URL> --addr <ADDR> --proof-path <PROOF_PATH>

Options:
  -S, --secret <SECRET>          The path to the wallet mnemonic
  -U, --rpc-url <RPC_URL>        RPC Url
      --addr <ADDR>              The deployed verifier address
      --proof-path <PROOF_PATH>  The path to the proof
  -h, --help                     Print help

```

For instance:

```bash
ezkl send-proof-evm -S ./mymnemonic.txt -U myethnode.xyz --addr 0xFFFF --proof-path my.snark
```