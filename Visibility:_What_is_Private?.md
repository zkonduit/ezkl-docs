---
icon: shield
order: 7
---
## Visibility

In `ezkl` there are four choices for Visibility: private, public, hashed, or encrypted, and three parts to chose these for: the model input, the model weights, and the model output (for 64 possible choices). The default setting is `private` input, `public` output, and `private` weights.

Visibility is controlled in the circuit settings (`settings.json` file), and is determined at setup time.

The question of what is private is very much related to the question of what we are proving. These questions tend to be a bit subtle and are really about designing the overall cryptosystem, of which your ezkl proof is a part. We provide these options to enable many different constructions. It can take some thought to determine which is right for your use case.

At a high level, mark those things `private` that you want to be secret to the prover, and allow the prover to change freely. Mark things `public` if you want them to be baked into the setup, and generally available (although see the comments about weight visibility below). Setting a part to `hashed` is a way to commit to it, and also a way to build bridges between proofs, making sure that a piece of data (input, weights, or output) is the same in multiple proofs. Hashed parts are also useful to reduce calldata size for smart contracts, and to allow something to be signed. Finally, setting a part to `kzgcommit` is a way to commit to it, and also a way to build bridges between proofs, making sure that a piece of data (input, weights, or output) is the same in multiple proofs.

Note that the proof file contains both the instance data (e.g. a hash of the model input and the actual output of the model) and the bytes of the cryptographic proof. These parts (instance data, proof bytes) are analogous to the message hash and signature in a digital signature.

## Data provenance, signatures, and linking data
A digital signature is a kind of zero knowledge proof. Ezkl can prove that a certain model says an image contains a cat, but you also have to think about whether that image is real (if that is important for your application). One technique to solve this *data provenance* problem is to use hashed visibility for the input image, and have a data source which separately signs the hash. Then the verifier can check the signature separately.

Putting it together, you would have two proofs. One, that Alice signed the (hash of the) image, using any signature algorithm on the Poseidon hash of the image. This can be computed and verified outside ezkl. Two, that the image with the given Poseidon hash contains a picture of a cat.

Then a verifier or verifier contract checks both the signature and the ezkl proof, and since the hash is the same, can be confident that the signature and the proof are "talking about" the same image.


## Weight visibility
When a model's weights are marked `public`, the weights are fixed at setup (circuit definition time). These weights are extractable from the proving key or the onnx file, but they can be kept private from the verifier as they are not part of the verifying key, proof, settings, or srs. Proofs can only be produced against the specific weights used at setup, so the verifier itself serves as a kind of implicit commitment to the weights. If you want to make an explicit commitment to the weights, for example to tie them to another model or sign them, use the hashed Visibility.

- Private: The weights are private to the prover, and can be chosen freely by the prover.
- Public: The weights are fixed by the setup, visible in the proving key, but not visible in the verifying key or verifier (although be aware of dictionary attacks).
- Hashed: The weights are private, but the hash of the weights is in the proof file, preventing the prover from changing the weights.
- KZGCommit: The kzg commitments to the weights are in the proof file, preventing the prover from changing the weights.

## Input visibility

- Private: The input is private to the prover, and can be chosen freely by the prover.
- Public: The input is part of the proof file, shared with the verifier.
- Hashed: The input is not sent in the proof file, but a Poseidon hash of the input is sent instead. The input is chosen by the prover, but it has to match the hash. The verifier cannot determine the input from the hash alone (although beware of dictionary attacks).
- KZGCommit: The kzg commitment to the input is in the proof file, preventing the prover from changing the input.


## Output visibility

- Private: The model output is private to the prover, and can be chosen freely by the prover.
- Public: The model output is part of the proof file, shared with the verifier.
- Hashed: The model output is not sent in the proof file, but a Poseidon hash of the output is sent instead. The verifier cannot determine the output from the hash alone (although beware of dictionary attacks).
- KZGCommit: The kzg commitment to the output is in the proof file, preventing the prover from changing the input.

--------------------

## Visibility Examples

This section can safely be skipped but might provide more clarity on some common options for visibility. Consider a function $f$ which given inputs $x,y$ and weight $c$ computes $f(x,y) = cx+y$. 

```python
class Model(nn.Module):
    def __init__(self, inplace=False):
        self.c = 3

    def forward(self, x, y):
        z = self.c * x + y
        return z
```

Notice that weights "$c$" are different from the inputs $x,y$ and output $z$. Weights are defined as part of the model (and will appear in the onnx). 

Suppose we set input public, weights fixed ("public"), and output public. This means that $c$ will be baked into the verifier and cannot be changed for this verifier. The witness data that will become part of the proof is $(x,y,z)$. The prover is proving that $cx+y = z$, and the verifier sees $x,y,$ and $z$, but not $c$. The setter-upper and the prover know $c$.

If we set input private, weights fixed ("public"), and output public, the witness data in the proof is $(z)$. The prover is proving that it knows secret $x,y$ such that $cx+y = z$. The setter-upper and the prover know $c$; the prover knows $x,y$; the verifier learns only $z$.

If we set input private, weights private, and output public, the witness data in the proof is $(z)$. The prover is proving that it knows secret $x,y,c$ such that $cx+y = z$ (practically the prover might set $c$ by changing a provided onnx file). The setter-upper does not know $c$, the prover knows $c,x,y$. The verifier learns only $z$.

If we set input private, weights hashed ($h = H(c)$), and output public, the witness data in the proof is $(h,z)$. The prover is proving that it knows secret $x,y,c$ such that $cx+y = z$ and $h=H(c)$. The setter-upper does not know $c$, the prover knows $c,x,y$. The verifier learns only $z$ and $h$. However, if $h$ was previously committed to, the prover can no longer freely choose the weight $c$.

Hashed weights and fixed ("public") weights are similar in that they both constrain the weights that the prover can use. They differ in that fixed weights bake the weights into the circuit at setup, whereas hashed weights can be determined at proof time. One consequence is that for fixed weights, to change the weights a new verifier would need to be deployed, whereas for hashed weights we could simply require a different hash target. However, we pay a performance penalty for the flexiblity of using the "dynamic" hashed weights rather than the "compiled" fixed weights.


