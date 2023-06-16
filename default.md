---
icon: rocket
order: 10
---
![](../assets/theThumbnail.jpg) 
# What is EZKL?

`ezkl` is a library and command-line tool for doing inference for deep learning models and other computational graphs in a zk-snark. It enables the following workflow:

1. Define a computational graph, for instance a neural network (but really any arbitrary set of operations), as you would normally in [pytorch](https://pytorch.org/docs/stable/index.html).
2. Export the final graph of operations as an [.onnx](https://onnx.ai/) file and some sample inputs to a `.json` file.
3. Point `ezkl` to the `.onnx` and `.json` files to generate a ZK-SNARK circuit with which you can prove statements such as:
> "I ran this publicly available neural network on some private data and it produced this output"

> "I ran my private neural network on some public data and it produced this output"

> "I correctly ran this publicly available neural network on some public data and it produced this output"

`ezkl` can be used directly from Python; [see this colab notebook](https://colab.research.google.com/drive/1XuXNKqH7axOelZXyU3gpoTOCvFetIsKu?usp=sharing) and the python bindings docs. [!ref](/python_bindings)

The rust API is also sufficiently flexible to enable you to code up a computational graph and resulting circuit from scratch. For examples on how to do so see the **library examples** in the repo. In the backend we use [Halo2](https://github.com/privacy-scaling-explorations/halo2) as a proof system. For more details on how to use `ezkl`, we invite you to explore the docs and check out the <a href="https://github.com/zkonduit/ezkl" target="_blank">repo</a>!

----------------------

## The life cycle of a proof

There are three steps in the life of an ezkl proof: Setup, Prove, and Verify. Each step is generally performed by a different party. 

### Setup 
Setup is invoked with `ezkl setup` at the cli or `ezkl.setup()` in Python. It defines what consitutes a proof and how that proof will be verified, setting the rules of the game. Setup is performed by the application developer, who then deploys the resulting artifacts to production. 

The inputs to setup are:
- the model (as an onnx file)
- the "params," (structured reference string) which are a common, public piece of cryptographic data shared among proofs of the same size
- various flags, settings, and options for tuning and added functionality

The outputs of setup are:
- the proving key
- the verification key
- the circuit params: serialized flags, settings, and options, and a few numbers that describe the shape of the resulting circuit 

### Prove
Prove, invoked with `ezkl prove` at the cli or `ezkl.prove()` in Python, is called by the prover, often on the client. The prover is making a claim that it knows some inputs (which might include model parameters), such that when the model (chosen during setup) is run on them, produces certain outputs. The prove function computes a cryptographic proof of that claim, which can then be believed by any verifier. 

The inputs to prove are:
- the data (an `input.json` file) containing the claim: an input, output pair such that model(input) = output (this output can be produced using the `forward` command)
- the model (as an onnx file)
- the proving key
- the params (structured reference string)
- the circuit params

The outputs of prove are:
- the proof file

### Verify 
Verify is invoked with `ezkl verify` at the cli, `ezkl.verify()` in Python, or from an Ethereum smart contract using the EVM verfier. It checks the correctness of the cryptographic proof produced by the prover.

The inputs to verify are:
- the proof file
- the verification key
- the circuit params
- the params (structured reference string)

`ezkl` can also produce an EVM or wasm verifier which takes only the proof as input.

----------------------

## Contributing 🌎

If you're interested in contributing and are unsure where to start, reach out to one of the maintainers on our [Telegram group](https://t.me/+QRzaRvTPIthlYWMx).

More broadly:

- Feel free to open up a [discussion topic](https://github.com/zkonduit/ezkl/discussions) to ask questions.

- See currently open issues for ideas on how to contribute.

- For PRs we use the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) naming convention.

