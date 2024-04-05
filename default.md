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

These proofs can be trusted by anyone with a copy of the verifier, and verified directly on Ethereum and compatible chains. `ezkl` can be used directly from Python; [see this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo.ipynb) and the python bindings docs. It can also be used from the command line.

`ezkl` can prove an MNIST-sized inference in less than a second and under 180mb of memory and verify it on the Ethereum Virtual Machine (or on the command line, or in the browser using wasm).

You can install the Python version with `pip install ezkl`.

`ezkl` can be used to move large and complex computations off-chain in a way that is easy to program (you can write your own functions in Python) and manage. You are not limited to a pre-defined set of functions, there is no limit on input size (using hashing), and there is no centralized sequencer.

For more details on how to use `ezkl`, we invite you to explore the docs and check out the <a href="https://github.com/zkonduit/ezkl" target="_blank">repo</a>, especially the <a href="https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/" target="_blank">notebooks.</a>

### Proving Backend (Lilith)

Running ZKML proofs can be computationally expensive. We've made the process easier by providing a backend service that can run the proofs for you.

If you're interested in using the Lilith backend, you can register your interest [here](https://ei40vx5x6j0.typeform.com/to/sFv1oxvb).


----------------------

## The life cycle of a proof

There are three steps in the life of an ezkl proof: Setup, Prove, and Verify. Each step is generally performed by a different party.

### Setup

Setup is invoked with `ezkl setup` at the cli or `ezkl.setup()` in Python. It defines what constitutes a proof and how that proof will be verified, setting the rules of the game. Setup is performed by the application developer, who then deploys the resulting artifacts to production.

The inputs to setup are:

- the model (as an onnx file)
- the structured reference string which is a common, public piece of cryptographic data shared among proof setups of the same size
- various flags, settings, and options for tuning and added functionality

The outputs of setup are:

- the proving key
- the verification key, and
- the circuit settings: serialized flags, settings, and options, and a few numbers that describe the shape of the resulting circuit.

Before setup can run, the settings need to be generated with `gen-settings` and optionally `calibrate-settings`, and the model must be compiled.

```python
ezkl.gen_settings()
ezkl.calibrate_settings()
res = ezkl.compile_model()
res = ezkl.setup()
```

### Prove

Prove, invoked with `ezkl prove` at the cli or `ezkl.prove()` in Python, is called by the prover, often on the client. The prover is making a claim that it knows some inputs (which might include model parameters), such that when the model (chosen during setup) is run on them, produces certain outputs. The prove command computes a cryptographic proof of that claim, which can then be believed by any verifier.

The inputs to prove are:

- the witness data for the claim: an (input, output) pair $(x,y)$ such that model(input) = output (this pair can be produced from $x$ using the `gen-witness` command)
- the model (as a compiled model file, made from an onnx file)
- the proving key
- the structured reference string, and
- the circuit settings.

The outputs of prove are:

- the proof file.

```python
res = ezkl.gen_witness()
res = ezkl.prove()
```

### Verify

`ezkl` can produce an EVM verifier contract which takes only the proof as input, and this is the normal use case.

```python
res = ezkl.create_evm_verifier()

# assuming anvil is running
res = ezkl.deploy_evm(
    address_path,
    sol_code_path,
    'http://127.0.0.1:3030'
)

res = ezkl.verify_evm(
    proof_path,
    addr,
    "http://127.0.0.1:3030"
)
```

Verification can also be invoked with `ezkl verify` at the cli, `ezkl.verify()` in Python, or with WASM. It checks the correctness of the cryptographic proof produced by the prover.

The inputs to (non-EVM) verify are:

- the proof file
- the verification key
- the circuit settings, and
- the structured reference string

----------------------

## Contributing 🌎

If you're interested in contributing and are unsure where to start, reach out to one of the maintainers on our [Telegram group](https://t.me/+QRzaRvTPIthlYWMx) or our [Discord](https://discord.gg/mqgdwdSgzA).

More broadly:

- Feel free to open up a [discussion topic](https://github.com/zkonduit/ezkl/discussions) to ask questions.

- See currently open issues for ideas on how to contribute.

- For PRs we use the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) naming convention.
