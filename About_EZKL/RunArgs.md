---
order: 1
---

### RunArgs

`RunArgs` are the fine-tuning parameters used to get more precise results when using `ezkl`. The majority of them have default values, so you typically don't see them in our basic instructions. This section will show you the various `RunArgs` you have at your disposal and how you can use them. To begin, here is a list of all the `RunArgs` we support and their flags (you can also take a look at these with `ezkl setup --help`):

> - Tolerance: `-T` | `--tolerance`
> - Scale: `-S` | `--scale`
> - Bits: `-B` | `--bits`
> - Logrows: `-K` | `logrows`
> - Batch Size: `--batch-size`
> - Public Inputs: `--public-inputs`
> - Public Outputs: `--public-outputs`
> - Public Parameters: `--public-params`
> - Pack Base: `--pack-base`
> - Allocated Constraints: `--allocated-constraints`

Let's go over each in detail with examples. Again, we'll be using the `1l_sigmoid` example under `examples/onnx`. 

##### Tolerance

Sometimes, quantization can throw off the output of our neural network. We need to quantize to keep model parameters within the finite field or our proof system, but with this margin for error, you might want to increase the range of values your output can be in order to verify. For example, let's say we are using a sigmoid layer with 2 values. The output should be `[0.5,0.5]` (would be whole numbers depending on `scale`, using decimals for simplicity), but due to quantization, the SNARK's output is `[0.4, 0.6]`. You know that this result is just from quantization, so you want to allow more values than *strictly* 0.5 and 0.5 to appear in the output. This is where percent tolerance comes in. You can set the tolerance for error on model outputs so that the proof verifies on outputs that aren't **exactly** what you're expecting. It's a very useful tool. You can use it like this:

```bash
ezkl setup -T 1.0  -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

This will give you a 1% tolerance on your outputs. 

##### Scale

As stated in the last section, `ezkl` quantizes neural networks to remain within the finite field of our proving system. This is why we have the `scale` RunArg. Scale serves as the number of digits for fixed point numbers after quantizing. For example, since the default is 7, after quantiation, the number `3.1415926` will be `31415926`, a number within the finite field of the elliptic curves halo2 uses. Let's say our model needs as much precision as possible. We can use a scale of  `13` to account for 13 decimal places:

```bash
ezkl setup -S 13 -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

##### Bits

Each halo2 cell represents a value in our circuit. By default, these are 16 bit values. However, `ezkl` allows us to use different numbers of bits for further customization and fine tuning. Let's say most of the values we use are under 100. In that case, we can use 8 bit cell values to save lots of space in memory. We do this with the `--bits` flag here:

```bash
ezkl setup -B 8 -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

##### Logrows

The `logrows` argument is the base 2 logarithm of the amount of rows that are in our halo2 circuit. it cannot exceed the value of `k` in our structure reference string (by default, this value is 17). You can read more about SRS in the first part of the Commands section. Let's say that when using `gen-srs` we created a SRS of size 23:

```bash
ezkl gen-srs --logrows=23 --params-path=kzg23.params
```

 This means we can setup circuits of any size less than or equal to that. Let's set up a circuit with 2^22 rows:

```bash
ezkl setup --logrows=22 -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg23.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

##### Batch Size

Typically in machine learning, models are trained and tested on batches of data. What if you want to prove that a model was run on a batch of data rather than a single input batch. You can do this with the `--batch-size` flag (default is 1). If you have a model with 3 batches of input data, `ezkl` defines a tensor over the first dimension of each of the input tensors and passes this through the proof as a whole. All three batches are then proven. 

##### Public Inputs

Set this flag to `true` with `--public-inputs=true` (default false) if you want your inputs to be public instance variables in the halo2 circuit. Otherwise, if you want private inputs, leave it alone or set it to `false`.

##### Public Outputs

Set this flag to `true` with `--public-outputs=true` (default true) if you want your outputs to be public instance variables in the circuit. By default, outputs are private. 

##### Public Parameters

Set this flag to `true` with `--public-params=true` if you want your circuit parameters to be public. This is false by default, but you may want to set it to `true` if you want others to reproduce the SNARK from a public model.

##### Pack Base

Sometimes, especially when verifying with the EVM, we want our outputs to take as little space as possible. The lengthy tensors can take up lots of memory when they really don't have to. `--pack-base` allows you to set a base for "[slot packing](https://fravoll.github.io/solidity-patterns/tight_variable_packing.html)" an output tensor using this strategy: **base ^ (scale + tensor)**. This allows us to create large field elements whose digits represent the values of our output tensor. Pretty cool. The default of `--pack-base` is 1, but feel free to use larger numbers, such as `--pack-base=3` if you are dealing with large output values.

##### Strategy

In the `Commands` section, we used an example of proof aggregation to aggregate two proofs into one. You'll notice that the `--strategy` we used is called `accum`. The other option for `--strategy` is `single` (the default). These are used to specify the proving strategy. If we are proving a single circuit, we can leave this alone. If we are proving with `aggregate`, use the `accum` strategy.

##### Allocated Constraints

Let's say you know the number of constraints from your model because you've generated a proof from it before. As a compute-saving hack, you can use `--allocated-constraints` to specify how many constraints your circuit uses so that `ezkl` doesn't have to calculate this. For example, let's say my model uses 5 constraints. By setting `--allocated-constraints` to 5, I save my prover compute and time by not having to run the dummy layout pass to compute this with each proof generation. It could be very useful for in-production SNARKs that need to shave off every second of proof generation time.