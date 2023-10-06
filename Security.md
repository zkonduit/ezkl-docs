---
icon: shield
order: -100
---
 
# Security

Zero knowledge machine learning, particularly in blockchain applications, is still a nascent field and should be used with caution. Because there have not yet been many production-ready projects, the potential attack vectors include both the usual and the mostly theoretical or unknown. `ezkl` has not been audited and we make no guarantees on its security.

Moreover, zkml is just one component of an overall cryptosystem, and that system as a whole has to be carefully thought out. Neural networks are not by themselves adequate hash functions; the whole point is that they are susceptible to differentiation!

Here are a few more things to worry about.

### AI/ML Security

There are several types of adversarial attacks on neural networks. [Gaussian Noise Injection](https://paperswithcode.com/paper/asymmetric-heavy-tails-and-implicit-bias-in), [Data poisoning](https://paperswithcode.com/task/data-poisoning), [Membership Inference Attacks](https://paperswithcode.com/paper/membership-inference-attacks-on-machine) (MIAs), and more are attack vectors that adversaries can use to corrupt your outputs. MIAs and others like it are especially hazardous when the aim of using zkml is to keep the model and its training data private.

Adversarial Training involves training your model with adversarial data so that edge cases are expected and accounted for. [CleverHans](https://github.com/cleverhans-lab/cleverhans) is a useful tool for discovering potential vulnerabilities in your model. For best security results, have an idea of the overall threat model of your neural net and its potential inputs.

### ZK Security

The goal of zero knowledge proof systems is to construct complete, sound proofs. Completeness is the highly probable assurance that any valid proof will verify. Soundness is the quality of the verifier (or parties representing the verifier) knowing that if a proof passes, it is more than likely a true statement. In some cases, such as those in [underconstrained circuits](https://eprint.iacr.org/2023/512.pdf), bad proofs can be generated that fool the verifier into passing a false statement. In this case, the vulnerability is not in the machine learning model itself, but in the SNARK constructed by `ezkl`.

`ezkl` is a compiler, so eventually should be less susceptible to such issues than a hand-written circuit, but it is still under active development.

Please reach out directly to let us know of any soundess issues you encounter.

###### Fuzzing

`ezkl` supports fuzzing over your model's edge inputs to test for potential vulnerabilities. Use your `input.json` and `network.onnx` files to run:

```bash
ezkl fuzz -D input.json -M network.onnx --transcript=evm --num-runs 10
```

Be sure to replace `num-runs` with the amount of fuzz testing rounds you want to do along with other parameters you are using to generate your circuit.

Thank you for using `ezkl`; please contact us if you have any comments on this documentation.
