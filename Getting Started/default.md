---
icon: code-square
order: 97
expanded: true
---
# Getting Started
Let's briefly preview the steps to generate and verify a model:

1. **Model Preparation**: prepare model and exports it as an ONNX file.
2. **Settings Generation**: run `ezkl.gen_settings()` to generate initial settings for the proof. Can optionally use `ezkl.calibrate_settings()` to optimize these settings.
3. **Model** **Compilation**: compile model using `ezkl.compile_model()`, which prepares it for the setup phase.
4. **Setup**: run `ezkl.setup()`, which defines the rules for the proof and verification. This step produces a proving key, a verification key, and circuit settings.
5. **Witness Generation**: generate the witness data (input-output pair) using `ezkl.gen_witness()`.
6. **Proof Generation**: create the cryptographic proof using `ezkl.prove()`, which takes the witness data, compiled model, proving key, and other necessary components.
7. [OPTIONAL] **Verifier Creation**: If the proof needs to be verified on-chain, the user can create an EVM verifier contract using `ezkl.create_evm_verifier()`.
8. [OPTIONAL] **Deployment**: For on-chain verification, the user deploys the verifier contract using `ezkl.deploy_evm()`.
9. **Verification**: The proof can be verified using one of three methods - on-chain using `ezkl.verify_evm()` with the deployed contract, off-chain via CLI using the `ezkl verify` command, or off-chain in Python using `ezkl.verify()`

Each of these steps and associated vocabulary are covered in detail in the following sections. Continue to the next section for installation.