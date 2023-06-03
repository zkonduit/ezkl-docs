---
icon: shield
order: -100
---
 
# Security
Zero knowledge machine learning, particularly in blockchain applications, is still a nascent field and should be used with precaution. Because there have not been many production-ready projects, the potential attack vectors are mostly theoretical or unknown. 

However, we ensure that if an ML model is secure, `ezkl` proofs will be sound and secure. Below we discuss best security practices for your neural networks and what we're doing on our end to ensure our proofs are secure.

### AI/ML Security
There are several types of adversarial attacks on neural networks. [Gaussian Noise Injection](https://paperswithcode.com/paper/asymmetric-heavy-tails-and-implicit-bias-in), [Data poisoning](https://paperswithcode.com/task/data-poisoning), [Membership Inference Attacks](https://paperswithcode.com/paper/membership-inference-attacks-on-machine) (MIAs), and more are attack vectors that adversaries can use to corrupt your ouputs. MIAs and others like it are especially hazardous when the aim of using zkml is to keep the model and its training data private. 

Adversarial Training involves training your model with adversarial data so that edge cases are expected and accounted for. [CleverHans](https://github.com/cleverhans-lab/cleverhans) is a useful tool for discovering potential vulnerabilities in your model. For best security results, have an idea of the overall threat model of your neural net and its potential inputs. 

### ZK Security
The goal of zero knowledge proof systems is to construct complete, sound proofs. Completeness is the highly probable assurance that any valid proof will verify. Soundness is the quality of the verifier (or parties representing the verifier) knowing that if a proof passes, it is more than likely a true statement. In some cases, such as those in [underconstrained circuits](https://eprint.iacr.org/2023/512.pdf), bad proofs can be generated that fool the verifier into passing a false statement. In this case, the vulnerability is not in the machine learning model itself, but in the SNARK constructed by `ezkl`. 

In this case, we would deeply appreciate an [issue](https://github.com/zkonduit/ezkl/issues) or [discussion](https://github.com/zkonduit/ezkl/discussions) regarding the bad proof. We will respond with alacrity.

### EZKL Security Tooling
We want to provide you with the best means of mitigating the potential of an attack on the SNARK of your model. This is why we are making security tools that allow you to test the robustness of your model. 

###### Fuzzing
`ezkl` supports fuzzing over your model's edge inputs to test for potential vulnerabilities. Use your `input.json` and `network.onnx` files to run:
```bash
ezkl fuzz -D input.json -M network.onnx --transcript=evm --num-runs 10
```
Be sure to replace `num-runs` with the amount of fuzz testing rounds you want to do along with other parameters you are using to generate your circuit. 

Thank you for using `ezkl`; please contact us if you have any security resources that should be included in this documentation.