---
order: 2
---

`RunArgs` are the fine-tuning parameters that give you more control over various aspects of `ezkl`. The majority of them have default values, so you typically don't see them in our basic instructions. This section will show you the various `RunArgs` you have at your disposal and how you can use them. To begin, here is a list of all the `RunArgs` we support and their flags (you can also take a look at these with `ezkl setup --help`):

> - Tolerance: `-T` | `--tolerance`
> - Scale: `-S` | `--scale`
> - Bits: `-B` | `--bits`
> - Logrows: `-K` | `--logrows`
> - Batch Size: `--batch-size`
> - Input Visibility: `--input-visibility`
> - Output Visibility: `--output-visibility`
> - Param Visibility: `--param-visibility`
> - Pack Base: `--pack-base`
> - Allocated Constraints: `--allocated-constraints`

Let's go over each in detail with examples. Again, we'll be using the `1l_sigmoid` example under `examples/onnx`. 

> Note: We use "computational graph" and "neural network" interchangeably. This is due to the fact that `ezkl` can be used for making SNARKs of any computational graph including neural networks. You can think of "computational graph" as your model in .onnx format.

### Tolerance

Sometimes, quantization can throw off the output of our computational graph. We need to quantize to represent the model using the finite field of our proof system, but you might want to explictly tolerate a small range of values in your output when verifying. For example, let's say we are using a sigmoid layer with 2 values. The output should be `[0.5,0.5]` (would be whole numbers depending on `scale`, using decimals for simplicity), but due to quantization, the SNARK's output is `[0.4, 0.6]`. You know that this result is just from quantization, so you want to allow more values than *strictly* 0.5 and 0.5 to appear in the output. This is where percent tolerance comes in. You can set the tolerance for error on model outputs so that the proof verifies on outputs that aren't **exactly** what you're expecting. You can use it like this:

```bash
ezkl setup -T 1.0  -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

This will give you a 1% tolerance on your outputs for this setup. 

### Scale

`ezkl` quantizes a floting point value to a fixed point value by multiplying by $2^{scale}$ and rounding. Then the numerator is stored. The default scale is 7. When two numerators of scale 7 are mutiplied, you get a number of scale 14. Scale can be adjusted with `-S` or `--scale`:

```bash
ezkl setup -S 6 -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

### Bits

Bits is the number of bits used in `ezkl`'s lookup tables for nonlinearities. The default is 16. We can adjust it with the `-B` or `--bits` flag here:

```bash
ezkl setup -B 14 -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

Scale and bits are related. Typically we want bits to be twice scale, plus a margin of error (of perhaps 2 or 3), since two scale 7 numbers multiplied become scale 14, and then adding several of them we get to an even larger scale. That's why the default bits is twice the default scale plus 2. `ezkl` will give some warnings and suggestions if your scale and bits are off. 

### Logrows

The `logrows` argument is the base 2 logarithm of the number of rows that are in our halo2 circuit. The default is 17. It cannot exceed the value of `k` in our structure reference string. You can read more about SRS in the first part of the Commands section. Bits must be at most one less than logrows (e.g. 16 when logrows is 17). Let's say that when using `gen-srs` we created a SRS of size 23:

```bash
ezkl gen-srs --logrows=23 --params-path=kzg23.params
```

 This means we can setup circuits of any size less than or equal to that. Let's set up a circuit with 2^22 rows:

```bash
ezkl setup --logrows=22 -M examples/onnx/1l_sigmoid/network.onnx --params-path kzg23.params --vk-path vk.key --pk-path pk.key --circuit-params-path circuit.params
```

### Batch Size

Typically in machine learning, models are trained and tested on batches of data. Perhaps you want to prove that a model was run on a batch of data rather than a single input batch. You can do this with the `--batch-size` flag (default is 1). 

### Input Visibility

Set this flag to `public` with `--public-inputs=public` (default `private`) if you want your inputs to be public instance variables in the halo2 circuit. Set it to `hashed` if you want the hash of your inputs to be public (with Poseidon). This can be useful for proving the validity or provenance of input data without necessarily making it public. 

### Output Visibility

Set this flag to `private` with `--public-outputs=private` (default `public`) if you want your outputs to be private variables in the circuit. By default, outputs are public. You can also set this to `hashed` if you want your outputs hashed.

### Param Visibility

Set this flag to `public` (default `private`) with `--public-params=public` if you want your circuit parameters to be public. You can also hash these by using `hashed`. These will give you the opportunity to prove you're using certain parameters to certain individuals, but not the public.

### Pack Base

Sometimes, especially when verifying with the EVM, we want our outputs to take as little space as possible.  `--pack-base` allows you to set a base to pack a tensor into an integer using powers of that base, like writing $[1,2,3]$ as 321 in base 10. 

### Strategy

In the `Commands` section, we used an example of proof aggregation to aggregate two proofs into one. You'll notice that the `--strategy` we used is called `accum`. The other option for `--strategy` is `single` (the default). These are used to specify the proving strategy. If we are proving a single circuit, we can leave this alone. If we are proving with `aggregate`, use the `accum` strategy.

### Allocated Constraints

Let's say you know the number of constraints from your model because you've generated a proof from it before. To save a bit of compute, you can use `--allocated-constraints` to specify how many constraints your circuit uses so that `ezkl` doesn't have to calculate this. For example, let's say my model uses 5 constraints. By setting `--allocated-constraints` to 5, I save my prover compute and time by not having to run the dummy layout pass to compute this with each proof generation. It could be very useful for in-production SNARKs that need to shave off every second of proof generation time.

By running `ezkl prove`, you can find the number of constraints your circuit has in `INFO`:

![img](../assets/constraints.png)
