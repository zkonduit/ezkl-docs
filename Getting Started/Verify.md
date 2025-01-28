---
icon: checklist
order: 93
---

The lifecycle of an EZKL proof consists of three core components: Setup, Prove, and Verify. This page focuses on the Verify phase, which checks the validity of a proof. Recall that:

- **Setup**: Defines proof parameters and generates keys (performed by developers)
- **Prove**: Generates a proof based on the setup (performed by users)
- **Verify**: Checks the validity of a proof (performed by verifiers)

### Process

The verification process involves a single step, **verify proof**. This can simply be done in-program or in-browser as demonstrated below.

### Parameters

To verify a proof, you'll need the following:

- `proof.json`: proof artifact
- `vk.key`: verification key
- `settings.json`: settings file
- `kzg.srs`: structured Reference String (SRS) file

Note that if you performed the Prove phase as instructed on the previous page, the CLI automatically pull from the correct paths and you should not have to specify the above parameters.

### Instructions for In-Program Verification

+++ Local CLI

```bash
ezkl verify
```

This verifies the zero-knowledge proof using the provided artifacts.

+++ Python

```python
import ezkl

ezkl.verify()
```

This verifies the zero-knowledge proof using the provided artifacts.

+++ JavaScript

```javascript
import { Engine } from "@ezkljs/engine";

const engine = new Engine();
await engine.verify();
```

This verifies the zero-knowledge proof using the provided artifacts.

+++ Remote CLI (Lilith)

```bash
archon job -a test verify
```

This verifies the zero-knowledge proof using the provided artifacts.
+++

### Outputs

The verification result is a boolean value:

- `true`: The proof is valid. The prover has demonstrated knowledge of inputs that satisfy the circuit constraints.
- `false`: The proof is invalid. This could be due to an incorrect proof or tampered input data.
