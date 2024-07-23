---
icon: package
order: 99
---

# Products
EZKL is a system for verifiable AI and analytics. The system includes two core products: the ezkl library and lilith orchestrator.
## EZKL

The ezkl library is free and open to install and run locally, designed for individuals and teams who want to explore compatibility or developer experience. It can be used in Python, JavaScript, and Rust programs. Note however that past version X.X.X, `ezkl` is not open for commercial usages without proper licensing.

## Lilith

Lilith is an orchestration platform for generating proofs at scale. It addresses a common issue we've observed: running more ambitious models or graphs with EZKL often maxes out developers' desktops and laptops. To expand what can be built using EZKL for everyone, we decided to provide some of our compute power.

Behind Lilith is a high-performance compute cluster dedicated to generating EZKL proofs on top-of-the-line hardware. The Lilith CLI serves as an orchestrator for triggering, scheduling, optimizing, and monitoring jobs. The key affordances of the platform include: 

1. **Enhanced Computational Power**: Enables processing of larger, more complex models.
2. **Reduced Latency**: Decreases proof generation time through distributed computing.
3. **Concurrent Processing**: Supports simultaneous proof generations for high-throughput scenarios.
4. **Seamless Integration**: Mirrors the `ezkl` CLI for easy scaling.
5. **Flexible Interaction**: Offers both CLI and REST API for varied integration needs.

### CLI Tool

We've built Lilith to mirror the experience of using EZKL locally as closely as possible. For example, if you're familiar with the `ezkl` CLI, a typical pipeline like:

```bash
bash
ezkl gen-witness
ezkl prove
```

looks like this using Lilith:

```bash
bash
lilith job gen-witness
lilith job prove
```

Each job returns a job-id which can then be queried using:

```bash
bash
lilith get -i <JOB_ID>
```

For more complex behavior, such as callbacks, you can use:

```bash
bash
lilith prove --callback_url <CALLBACK_URL>
```

This will send the proof as a JSON-encoded object to `CALLBACK_URL` upon job completion, allowing you to trigger jobs and get the result upon completion without needing to poll.

> Effectively, EZKL is for **local** usage and Lilith is for **remote** usage. For the remaining documentation, we will demonstrate equivalent commands for EZKL CLI and Lilith CLI, as well as for Python and JavaScript environments.

### REST API

For more complex scripting, Lilith exposes a REST API. This allows you to programmatically trigger jobs, chain new jobs together, and add new data for the worker to operate upon, all in a single call. 

For instance, to chain a `gen-witness` call and `prove` call together on new data:

```bash
bash
curl -XPOST -H "Content-type: application/json" -d '{
    "commands": [
        {
            "ezkl_command": {
                "GenWitness": {
                    "data": "input.json",
                    "compiled_circuit": "model.compiled",
                    "output": "witness.json"
                }
            }
        },
        {
            "ezkl_command": {
                "Prove": {
                    "witness": "witness.json",
                    "compiled_circuit": "model.compiled",
                    "pk_path": "pk.key",
                    "proof_path": "proof.json",
                    "proof_type": "Single",
                    "check_mode": "UNSAFE"
                }
            }
        }
    ],
    "data": [
        {
            "input_data": [
                [1.514470100402832, 1.519423007965088, 1.5182757377624512,
                 1.5262789726257324, 1.5298409461975098]
            ]
        }
    ]
}' 'http://<LILITH_URL>/recipe?callback_url=<CALLBACK_URL>'

```

### Usage Tiers

**Public Cluster**

Free to use with certain caps on daily usage and overall proof storage. Intended for individuals, researchers, and teams who want to build simple applications or test EZKL's performance.

**Enterprise Cluster**

Please reach out to the EZKL team if you are interested in enterprise access. For access without daily usage and proof storage limitations. Useful for commercial applications and large-scale systems incorporating EZKL.

**Private Deployment**

Please reach out to the EZKL team if you are interested in private deployment. A private deployment of the Lilith system, relevant for teams building application components with EZKL or miners/validators generating EZKL proofs.