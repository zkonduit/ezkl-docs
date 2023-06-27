---
icon: workflow
order: 7
---
## Visibility

In `ezkl` there are four choices for Visibility: private, public, hashed, or encrypted, and three parts to chose these for: the model input, the model weights, and the model output (for 64 possible choices). The default setting is `private` input, `public` output, and `private` weights.

The question of what is private is very much related to the question of what we are proving. These questions tend to be a bit subtle and are really about designing the overall cryptosystem, of which your zkml proof is a part. We provide these options to enable many different constructions, and we can help you determine which is right for you use case.

At a high level, mark those things `private` that you want to be secret to the prover, and allow the prover to change freely. Mark things `public` if you want them to be baked into the setup, and generally available (although see the comments about weight visibility below). Setting a part to `hashed` is a way to commit to it, and also a way to build bridges between proofs, making sure that a piece of data is the same in multiple proofs (including digital signatures). Hashed parts are especially useful to reduce calldata size for smart contracts. Finally making a part `encrypted` proves encryption inside the circuit, which is useful for some constructions such as marketplaces with escrow contracts.



## Input visibility

### Private

### Public

### Hashed

### Encrypted




## Weight visibility
When a model's weights are marked `public`, the weights are fixed at setup (circuit definition time). These weights are extractable from the proving key or the onnx file, but they can be kept private from the verifier as they are not part of the verifying key, proof, settings, or srs. Proofs can only be produced against the specific weights used at setup, so the verifier itself serves as a kind of implicit commitment to the weights. If you want to make an explicit commitment to the weights, for example to tie them to another model or sign them, use the hashed Visibility.

### Private

### Public

### Hashed

### Encrypted


## Output visibility

### Private

### Public

### Hashed

### Encrypted

