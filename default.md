---
icon: rocket
order: 100
---

# The EZKL System

**EZKL is is a developer-friendly system for verifiable AI and analytics.**

Analytics can be descriptive (aggregation, visualization), diagnostic (mining, statistical, querying), predictive (machine learning, time series), and prescriptive (optimization, simulation, decision). Verifiability is the ability to confirm the use of designated mathematical steps, even if you are not the executing machine.

This is useful in three core scenarios:

1. **Running a public model on private data.** For example, a research paper includes a model for detecting indicators for a specific disease. Since the model must've been trained on sensitive patient data, the author can use `ezkl` to prove that the benchmarks are true for the committed model and private data, without a reviewer needing to access that data.
2. **Running a private model on public data**. For example, a hedge fund leverages a model for identifying and trading on financial opportunities. Since the model is proprietary, a limited partner or investor may provide benchmarking data to the fund, and the fund can use `ezkl` to prove that their model is as accurate or performant as reported.
3. **Running a public model on public data**. For example, there is a computationally limited machine like a blockchain. You are a portfolio manager and want to execute rebalancing behalf of your client according to market data. While your rebalancing model may be public and the market data is public, this is too computationally expensive to calculate the adjusted values on-chain. The PM can use `ezkl` to apply the rebalancing model off-chain, then prove to the machine that they used that particular model and upon confirmation trigger execution on-chain. This scenario clearly affords automation in addition to verifiability

See that in all scenarios, the actor with ownership over model execution can **prove** that they are acting in good faith. Their actions are **verifiable** to an recipient of the results.

## How This Works

The underlying technology enabling this verifiability is known as zero knowledge cryptography. The proofs generated are called zero knowledge proofs (ZKPs), which enable one party (the prover) to prove to another (the verifier) that a statement is true without revealing any extra information.

Traditionally, creating ZKPs requires manually designing circuits using domain-specific languages like Circom or CirC. This process is particularly challenging for complex AI/ML models with thousands of gates.

`ezkl` automates ZKP generation for these complex computations as following:

1. **Input**: User provides a model (e.g., neural network) in ONNX format.
2. **Circuit Generation**: `ezkl` automatically converts the model into a ZKP-compatible circuit.
3. **Proof Creation**: `ezkl` generates a proof of correct model execution.
4. **Verification**: Anyone with the verification key can verify the proof.

From this high level overview, you can observe a few distinct design choices. First, `ezkl` utilizes a highly improved version of Halo2, which is a zero-knowledge proving system developed by Zcash, as its underlying cryptographic framework. This was selected for it's "public auditing" through years of previous development and overall completeness. However, `ezkl` is not limited to the Halo2 proving system and is incorporating additional systems.

Further, as commonly done in ZK research, models' computational tasks are represented as circuits composed of logical gates, allowing for efficient ZKP generation. In order to convert computational tasks to circuits, we specifically support models in the ONNX format. ONNX is also used widely in the broader machine learning community.

Finally, `ezkl` supports a broad range of model architectures:

- **Tree-based Models**: Decision trees and random forests are compiled by representing decision nodes as conditional statements in the circuit.
- **Transformers**: These are handled by breaking down attention mechanisms and feed-forward layers into basic matrix operations that can be represented in the circuit.
- **Other Architectures**: `ezkl` can theoretically compile any model representable in ONNX, including CNNs, RNNs, and custom architectures, by decomposing them into basic mathematical operations.

This flexibility allows `ezkl` to support a wide range of AI/ML applications, from traditional statistical models to cutting-edge deep learning architectures.

## Developer Experience

EZKL is designed with a core aim: to enable smooth developer experiences in working with zero-knowledge proofs. Developers should not require knowledge of cryptography or zero-knowledge proofs to use the library. 

### No Prerequisite Knowledge Required

One of EZKL's key strengths is its accessibility. This is possible through the `ezkl` compiler. You don't need:

- Cryptography expertise
- Zero-knowledge proof theory understanding
- Experience writing constraints
- Knowledge of circuit design
- Skills in orchestrating concurrent systems
- Advanced DevOps knowledge

EZKL automates the complex underlying processes, allowing you to focus on your application logic.

### Multiple Language Support

EZKL supports different approaches to development, catering to different preferences and use cases:

1. **Scripting/CLI.** For those who prefer a command-line interface, EZKL offers a comprehensive CLI. This is ideal for quick operations and scripting.
    
    ```bash
    ezkl gen-settings
    ezkl compile-model
    ezkl prove
    ```
    
2. **Python.** Python developers can leverage EZKL's Python bindings for integration into their existing workflows.
    
    ```python
    import ezkl
    settings = ezkl.gen_settings()
    ezkl.compile_model("model.onnx", "compiled_model.ezkl", settings)
    proof = ezkl.prove("witness.json", "compiled_model.ezkl", "pk.key")
    ```
    
3. **JavaScript.** For web developers and Node.js enthusiasts, EZKL provides JavaScript bindings.
    
    ```jsx
    const ezkl = require('ezkl');
    const settings = await ezkl.genSettings();
    await ezkl.compileModel('model.onnx', 'compiled_model.ezkl', settings);
    const proof = await ezkl.prove('witness.json', 'compiled_model.ezkl', 'pk.key');
    ```
    
4. **Rust.** For those who need low-level control and maximum performance, EZKL can be used directly in Rust projects.
    
    ```rust
    use ezkl;
    let settings = ezkl::gen_settings()?;
    ezkl::compile_model("model.onnx", "compiled_model.ezkl", &settings)?;
    let proof = ezkl::prove("witness.json", "compiled_model.ezkl", "pk.key")?;
    ```

### Abstraction of Complexity

EZKL abstracts away the complexities typically associated with zero-knowledge proofs:

1. **No Manual Circuit Design**: Instead of hand-writing circuits, you can use standard machine learning models in ONNX format. EZKL automatically converts these into ZKP-compatible circuits.
2. **Automated Constraint Generation**: EZKL handles the generation of constraints, eliminating the need for developers to manually define them.
3. **Simplified Proof Generation**: The process of generating proofs is streamlined into simple function calls or CLI commands, hiding the intricate cryptographic operations.
4. **Job Orchestration**: When using Lilith, the orchestration of concurrent proof generation is managed for you, eliminating the need to design and implement complex distributed systems.
5. **Managed DevOps**: With Lilith's cloud offering, much of the DevOps complexity is abstracted away. You don't need to set up and maintain high-performance hardware for proof generation.

## Scaling Proof Generation

`ezkl` is designed not only as a proof compiler but also as a comprehensive system for generating zero-knowledge proofs at scale. It addresses the varying time sensitivities and computational demands across different use cases, from real-time financial operations to concurrent model proving.

While `ezkl` is a highly performant system [LINK BENCHMARKS], it primarily operates locally on a user's machine. This local execution can be limiting for very large models or when rapid, concurrent verifications are needed. To address these scalability challenges, we've developed Lilith.

**Lilith is a high-performance compute cluster dedicated to generating EZKL proofs on top-of-the-line hardware.** It serves as a remote orchestrator, allowing users to offload the computational burden of proof generation. Key benefits of using Lilith include:

- **Enhanced Computational Power**: Enables processing of larger, more complex models.
- **Reduced Latency**: Decreases proof generation time through distributed computing.
- **Concurrent Processing**: Supports simultaneous proof generations for high-throughput scenarios.
- **Seamless Integration**: Mirrors the `ezkl` CLI for easy scaling.
- **Flexible Interaction**: Offers both CLI and REST API for varied integration needs.

While `ezkl` continues to be optimized for local use, Lilith extends its capabilities to cloud-scale operations. This dual approach ensures that EZKL can meet a wide range of performance and scalability requirements, from individual developers to enterprise-level applications.

For more details on how to use the EZKL system, we invite you to explore the remaining documentation and check out the <a href="https://github.com/zkonduit/ezkl" target="_blank">original repository</a>, example <a href="https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/" target="_blank">Jupyter notebooks</a>, and our <a href="https://www.ezkl.xyz">website</a>.

----------------------

## Contribute

If you're interested in contributing and are unsure where to start, reach out to one of the maintainers on our [Telegram group](https://t.me/+QRzaRvTPIthlYWMx) or our [Discord](https://discord.gg/mqgdwdSgzA).

More broadly:

- Feel free to open up a [discussion topic](https://github.com/zkonduit/ezkl/discussions) to ask questions.
- See currently open issues for ideas on how to contribute.
- For PRs we use the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) naming convention.
