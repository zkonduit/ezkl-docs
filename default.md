---
icon: rocket
order: 4
---

# What is EZKL?

`ezkl` is a library and command-line tool for doing inference for deep learning models and other computational graphs in a zk-snark. It enables the following workflow:

1. Define a computational graph, for instance a neural network (but really any arbitrary set of operations), as you would normally in pytorch or tensorflow.
2. Export the final graph of operations as an [.onnx](https://onnx.ai/) file and some sample inputs to a `.json` file.
3. Point `ezkl` to the `.onnx` and `.json` files to generate a ZK-SNARK circuit with which you can prove statements such as:
> "I ran this publicly available neural network on some private data and it produced this output"

> "I ran my private neural network on some public data and it produced this output"

> "I correctly ran this publicly available neural network on some public data and it produced this output"

The rust API is also sufficiently flexible to enable you to code up a computational graph and resulting circuit from scratch. For examples on how to do so see the **library examples** section below.

In the backend we use [Halo2](https://github.com/privacy-scaling-explorations/halo2) as a proof system.

For more details on how to use `ezkl`, we invite you to explore the docs and check out the <a href="https://github.com/zkonduit/ezkl" target="_blank">repo</a>!

----------------------

## Contributing 🌎

If you're interested in contributing and are unsure where to start, reach out to one of the maintainers:

* dante (alexander-camuto)
* jason (jasonmorton)

More broadly:

- Feel free to open up a discussion topic to ask questions.

- See currently open issues for ideas on how to contribute.

- For PRs we use the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) naming convention.

