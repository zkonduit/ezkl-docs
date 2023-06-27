---
icon: rocket
order: 100
---

# What is EZKL?

## EZKL makes zero-knowledge easier
`ezkl` takes a high-level description of your program and sets up a zero-knowledge prover and verifier. Our focus is on programs that are expressed as [pytorch](https://pytorch.org/docs/stable/index.html) AI/ML models and other computational graphs. After setup, the prover can prove statements such as the following.

> "I ran this publicly available neural network on some private data and it produced this output"

> "I ran my private neural network on some public data and it produced this output"

> "I correctly ran this publicly available neural network on some public data and it produced this output"

`ezkl` can be used as a command-line tool, or directly from Python; [see this colab notebook](https://colab.research.google.com/drive/1XuXNKqH7axOelZXyU3gpoTOCvFetIsKu?usp=sharing) and the python bindings docs. [!ref](/python_bindings)

`ezkl` can prove an MNIST-sized inference in less than a second and under 180mb of memory and verify on the Ethereum Virtual Machine (or on the command line, or in the browser using wasm). 

For more details on how to use `ezkl`, we invite you to explore the docs and check out the <a href="https://github.com/zkonduit/ezkl" target="_blank">repo</a>!

----------------------


## Zero-knowledge proofs
A zero-knowledge proof is a programmable digital signature. In a normal digital signature scheme, we have a secret key, a public message, and a signature algorithm that was set up in advance and everyone knows. These are applied to the inputs to produce a signature and a public message, which is then verified by a counterparty (such as a blockchain). 
![](../assets/before.png) 
A zero-knowledge proof allows us to replace the secret key and public message with arbitrary private and public inputs, and it lets us run any program we like on them, not just a fixed signature algorithm.

![](../assets/after.png) 
Just as with a digital signature, there are three parties: one to define the setup, one to prove, and one to verify.

## The life cycle of a proof

There are three steps in the life of an ezkl proof: Setup, Prove, and Verify. Each step is generally performed by a different party. 

### Setup 
Setup is invoked with `ezkl setup` at the cli or `ezkl.setup()` in Python. It defines what consitutes a proof and how that proof will be verified, setting the rules of the game. Setup is performed by the application developer, who then deploys the resulting artifacts to production. 

The inputs to setup are:
- the model (as an onnx file)
- the structured reference string which is a common, public piece of cryptographic data shared among proofs of the same size
- various flags, settings, and options for tuning and added functionality

The outputs of setup are:
- the proving key
- the verification key, and
- the circuit settings: serialized flags, settings, and options, and a few numbers that describe the shape of the resulting circuit.

### Prove
Prove, invoked with `ezkl prove` at the cli or `ezkl.prove()` in Python, is called by the prover, often on the client. The prover is making a claim that it knows some inputs (which might include model parameters), such that when the model (chosen during setup) is run on them, produces certain outputs. The prove command computes a cryptographic proof of that claim, which can then be believed by any verifier. 

The inputs to prove are:
- the witness data for the claim: an (input, output) pair $(x,y)$ such that model(input) = output (this pair can be produced from $x$ using the `gen-witness` command)
- the model (as an onnx file)
- the proving key
- the structured reference string, and
- the circuit settings.

The outputs of prove are:
- the proof file.

### Verify 
Verify is invoked with `ezkl verify` at the cli, `ezkl.verify()` in Python, from an Ethereum smart contract using the EVM verfier, or with WASM. It checks the correctness of the cryptographic proof produced by the prover.

Conceptually the inputs to verify are:
- the proof file
- the verification key
- the circuit settings, and
- the structured reference string

but `ezkl` can produce an EVM verifier which takes only the proof as input, and this is the normal use case.

----------------------

## Contributing 🌎

If you're interested in contributing and are unsure where to start, reach out to one of the maintainers on our [Telegram group](https://t.me/+QRzaRvTPIthlYWMx).

More broadly:

- Feel free to open up a [discussion topic](https://github.com/zkonduit/ezkl/discussions) to ask questions.

- See currently open issues for ideas on how to contribute.

- For PRs we use the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) naming convention.

