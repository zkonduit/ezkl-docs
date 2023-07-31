---
order: 2
---

`RunArgs` are the fine-tuning parameters that give you more control over various aspects of `ezkl`. The majority of them have default values, so you typically don't see them in our basic instructions. They include:

> - Tolerance: `-T` | `--tolerance`
> - Scale: `-S` | `--scale`
> - Bits: `-B` | `--bits`
> - Logrows: `-K` | `--logrows`
> - Batch Size: `--batch-size`
> - Input Visibility: `--input-visibility`
> - Output Visibility: `--output-visibility`
> - Param Visibility: `--param-visibility`
> - Allocated Constraints: `--allocated-constraints`

they can be edited in the settings file, or passed as arguments to `ezkl gen-settings`. 

```
Usage: ezkl gen-settings [OPTIONS] --model <MODEL>

Options:
  -M, --model <MODEL>
          The path to the .onnx model file
  -O, --settings-path <SETTINGS_PATH>
          Path to circuit_settings file to output [default: settings.json]
  -T, --tolerance <TOLERANCE>
          The tolerance for error on model outputs [default: 0]
  -S, --scale <SCALE>
          The denominator in the fixed point representation used when quantizing [default: 7]
  -B, --bits <BITS>
          The number of bits used in lookup tables [default: 16]
  -K, --logrows <LOGROWS>
          The log_2 number of rows [default: 17]
      --batch-size <BATCH_SIZE>
          The number of batches to split the input data into [default: 1]
      --input-visibility <INPUT_VISIBILITY>
          Flags whether inputs are public, private, hashed [default: private]
      --output-visibility <OUTPUT_VISIBILITY>
          Flags whether outputs are public, private, hashed [default: public]
      --param-visibility <PARAM_VISIBILITY>
          Flags whether params are public, private, hashed [default: private]
      --allocated-constraints <ALLOCATED_CONSTRAINTS>
          the number of constraints the circuit might use. If not specified, this will be calculated using a 'dummy layout' pass
```

### Tolerance

We need to quantize to represent the model using the finite field of our proof system, and this can introduce numerical error. You might want to explictly tolerate a small range of values in your output when verifying. A value of 1.0 is 1% tolerance. 

### Scale

`ezkl` quantizes a floting point value to a fixed point value by multiplying by $2^{scale}$ and rounding. Then the numerator is stored. The default scale is 7. When two numerators of scale 7 are mutiplied, you get a number of scale 14. Scale can be adjusted with `-S` or `--scale`:

### Bits

Bits is the number of bits used in `ezkl`'s lookup tables for nonlinearities. The default is 16. We can adjust it with the `-B` or `--bits` flag.

Scale and bits are related. Typically we want bits to be twice scale, plus a margin of error (of perhaps 2 or 3), since two scale 7 numbers multiplied become scale 14, and then adding several of them we get to an even larger scale. That's why the default bits is twice the default scale plus 2. `ezkl` will give some warnings and suggestions if your scale and bits are off. Or just use `calibrate-settings`.

### Logrows

The `logrows` argument is the base 2 logarithm of the number of rows that are in our halo2 circuit. The default is 17. It cannot exceed the value of `k` in our structured reference string. You can read more about SRS in the first part of the Commands section. Bits must be at most one less than logrows (e.g. 16 when logrows is 17). 

### Batch Size

Perhaps you want to prove that a model was run on a batch of data rather than a single input. You can do this with the `--batch-size` flag (default is 1). 

### Input Visibility

Set this flag to `public` with `--public-inputs=public` (default `private`) if you want your inputs to be public instance variables in the halo2 circuit. Set it to `hashed` if you want the hash of your inputs to be public (with Poseidon). This can be useful for proving the validity or provenance of input data without necessarily making it public. 

### Output Visibility

Set this flag to `private` with `--public-outputs=private` (default `public`) if you want your outputs to be private variables in the circuit. By default, outputs are public. You can also set this to `hashed` if you want your outputs hashed.

### Param Visibility

Set this flag to `public` (default `private`) with `--public-params=public` if you want your circuit parameters to be public. You can also hash these by using `hashed`. 

### Strategy

In the `Commands` section, we used an example of proof aggregation to aggregate two proofs into one. You'll notice that the `--strategy` we used is called `accum`. The other option for `--strategy` is `single` (the default). These are used to specify the proving strategy. If we are proving a single circuit, we can leave this alone. If we are proving with `aggregate`, use the `accum` strategy.

