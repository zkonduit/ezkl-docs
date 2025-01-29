---
icon: gear
order: 95
---

The lifecycle of an EZKL proof consists of three core components: Setup, Prove, and Verify. This page focuses on the Setup phase, which defines the rules for proof generation and verification. Recall that:

- **Setup**: Defines proof parameters and generates keys (performed by developers)
- **Prove**: Generates a proof based on the setup (performed by users)
- **Verify**: Checks the validity of a proof (performed by verifiers)

### Process

The setup process involves the following steps:

1. **Generate settings**: Creates a configuration file with default parameters for the circuit.
2. **Calibrate settings (optional)**: Fine-tunes the circuit parameters to optimize for either accuracy or resource usage.
3. **Compile the model**: Converts the ONNX model into a format optimized for zero-knowledge proofs.
4. **Run setup**: Generates the cryptographic keys needed for proving and verifying.

### Parameters

To perform the setup, you'll need to provide and/or create the following:

#### ONNX File

For PyTorch:

```python
import torch.onnx

dummy_input = torch.randn(1, 3, 224, 224)  # Adjust input dimensions as needed
torch.onnx.export(model, dummy_input, "network.onnx", opset_version=10)
```

For TensorFlow:

```python
import tf2onnx

onnx_model, _ = tf2onnx.convert.from_keras(model)
with open("network.onnx", "wb") as f:
    f.write(onnx_model.SerializeToString())
```

#### Configuration Options

These are defined in the `settings.json` file, which is generated and optionally calibrated during the setup process.

### Instructions

+++ Local CLI

1. Generate settings:

```bash
ezkl gen-settings
```

This creates a settings.json file with default parameters based on your ONNX model.

2. Calibrate settings (optional):

```bash
ezkl calibrate-settings
```

This optimizes the settings for resource usage. You can also use `--target accuracy` to optimize for accuracy, and `--target resources` to optimize for resources.

3. Compile model:

```bash
ezkl compile-circuit
```

This converts your ONNX model into an optimized format for zero-knowledge proofs, creating the `network.ezkl` file.

4. Run setup:

```bash
ezkl setup
```

This generates the cryptographic keys needed for proving and verifying.

+++ Python

1. Generate settings:

```python
import ezkl

ezkl.gen_settings("network.onnx")
```

This creates a `settings.json` file with default parameters based on your ONNX model.

2. Calibrate settings (optional):

```python
ezkl.calibrate_settings("network.onnx", "settings.json", target="resources")
```

This optimizes the settings for resource usage. You can also use `target="accuracy"` to optimize for accuracy.

3. Compile model:

```python
ezkl.compile_circuit("network.onnx", "network.ezkl", "settings.json")
```

This converts your ONNX model into an optimized format for zero-knowledge proofs, creating the `network.ezkl` file.

4. Run setup:

```python
ezkl.setup("network.ezkl", "vk.key", "pk.key", "kzg.srs")
```

This generates the cryptographic keys needed for proving and verifying.

+++ JavaScript

1. Generate settings:

```javascript
import { Engine } from "@ezkljs/engine";

const engine = new Engine();
await engine.genSettings("network.onnx");
```

This creates a `settings.json` file with default parameters based on your ONNX model.

2. Calibrate settings (optional):

```javascript
await engine.calibrateSettings("network.onnx", "settings.json", {
  target: "resources",
});
```

This optimizes the settings for resource usage. You can also use `{ target: "accuracy" }` to optimize for accuracy.

3. Compile model:

```javascript
await engine.compileCircuit("network.onnx", "network.ezkl", "settings.json");
```

This converts your ONNX model into an optimized format for zero-knowledge proofs, creating the `network.ezkl` file.

4. Run setup:

```javascript
await engine.setup("network.ezkl", "vk.key", "pk.key", "kzg.srs");
```

This generates the cryptographic keys needed for proving and verifying.

+++ Remote CLI (Lilith)

1. Upload your files,

```bash
archon create-artifact -a test -i input.json -m network.onnx -c calibration.json
```

2. Generate settings:

```bash
archon job -a test gen-settings
```

This creates a `settings.json` file with default parameters based on your ONNX model.

3. Calibrate settings (optional):

```bash
archon job -a test calibrate-settings
```

This optimizes the settings for resource usage. You can also use `--target accuracy` to optimize for accuracy, and `--target resources` to optimize for resources.

4. Compile model:

```bash
archon job -a test compile-circuit
```

This converts your ONNX model into an optimized format for zero-knowledge proofs, creating the `network.ezkl` file.

4. Run setup:

```bash
archon job -a test setup
```

This generates the cryptographic keys needed for proving and verifying.
+++

### Outputs

The setup process will generate the following:

- `network.onnx`: model in ONNX format
- `settings.json`: generated file which is optionally calibrated during the setup process
- `network.ezkl`: compiled circuit created from your ONNX model in step 3.
- `kzg.srs`: Structured Reference String (SRS), which you can download using `ezkl get-srs`.
- `vk.key` and `pk.key`: verification and proving keys generated during the setup process
