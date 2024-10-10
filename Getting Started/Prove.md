---
icon: device-desktop
order: 94
---
The lifecycle of an EZKL proof consists of three core components: Setup, Prove, and Verify. This page focuses on the Prove phase, which defines the rules for proof generation and verification. Recall that:
- **Setup**: Defines proof parameters and generates keys (performed by developers)
- **Prove**: Generates a proof based on the setup (performed by users)
- **Verify**: Checks the validity of a proof (performed by verifiers)

### Process
The setup process involves the following steps:
1. **Generate witness**: Creates a witness file from the input data and the compiled model. 

A witness in this context is a comprehensive record of all the intermediate values and computations performed when running the input through the machine learning model. It includes:
* The input data
* All intermediate layer outputs
* The final output of the model
* Any additional data required for the proof (e.g., random numbers used in the computation)

The witness serves as a "trace" of the computation, allowing the prover to demonstrate knowledge of all the steps involved in running the input through the model without revealing the specific values.

2. **Generate proof**: Uses the witness, proving key, and other artifacts to create a zero-knowledge proof.

### Parameters
To generate a proof, you'll need the following:
- `network.ezkl`: compiled circuit file
- `input.json`: input data file 
- `pk.key`: proving key
- `settings.json`: settings file
- `kzg.srs`: structured Reference String (SRS) file

Note that if you performed the Setup phase as instructed on the previous page, the CLI automatically pull from the correct paths and you should not have to specify the above parameters.

### Instructions
+++ Local CLI

1. Generate witness:
```bash
ezkl gen-witness
```
This creates a witness file from your input data and compiled model.

2. Generate proof:
``` bash
ezkl prove
```
This generates a zero-knowledge proof using the witness and other artifacts.

+++ Python

1. Generate witness:
```python
import ezkl

ezkl.gen_witness()
```
This creates a witness file from your input data and compiled model.

2. Generate proof:
```python
ezkl.prove()
```
This generates a zero-knowledge proof using the witness and other artifacts.

+++ JavaScript

1. Generate witness:
```javascript
import { Engine } from '@ezkljs/engine';

const engine = new Engine();
await engine.genWitness();
```
This creates a witness file from your input data and compiled model.

2. Generate proof:
```javascript
await engine.prove();
```
This generates a zero-knowledge proof using the witness and other artifacts.

+++ Remote CLI (Lilith)

1. Generate witness:
```bash
archon job -a test gen-witness
```
This creates a witness file from your input data and compiled model.

2. Generate proof:
```bash
archon job -a test prove
```
This generates a zero-knowledge proof using the witness and other artifacts.

+++
### Outputs
The proof generation process will produce the following:
- `witness.json`: intermediate values computed during the execution of the circuit
- `proof.json`: zero-knowledge proof artifact