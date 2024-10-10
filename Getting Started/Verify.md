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

However, there is the option of verification on-chain as well (for smart contract applications). Verifying proofs on-chain offers several advantages:
- **Smart Contract Integration**: On-chain verification allows other smart contracts to react to valid proofs, enabling complex decentralized applications.
- **Gas Efficiency**: While initial deployment of the verifier contract may be costly, subsequent verifications are typically much cheaper than performing the full computation on-chain.
- **Reusability**: Once deployed, the verifier contract can be used multiple times, amortizing the initial deployment cost and making it persistently accessible to other calling contracts.
- **Privacy**: On-chain verification maintains the privacy guarantees of the zero-knowledge proof, allowing for confidential data processing in public blockchains

If you are integrating with smart contracts, consider the following:
- `Verifier Contract`: The EZKL-generated verifier contract contains the logic to verify proofs. This contract should be deployed once and can be used for multiple verifications.
- `Proof Submission`: Design your smart contracts to accept proof data as input. This typically includes the proof itself and any public inputs.
- `Result Handling`: The verifier contract's verify function returns a boolean. Your contract should check this result and act accordingly.
- `Gas Optimization`: Consider implementing batched proof verification if your use case involves verifying multiple proofs in a single transaction.
- `Upgradability`: If you anticipate changes to your ML model or proof system, consider implementing upgradeable smart contracts.

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
import { Engine } from '@ezkljs/engine';

const engine = new Engine();
await engine.verify();
```
This verifies the zero-knowledge proof using the provided artifacts.

+++ Remote CLI (Lilith)

```bash
lilith verify
```
This verifies the zero-knowledge proof using the provided artifacts.
+++
### Instructions for On-Chain Verification
+++ Ethereum
1. Generate Solidity verifier contract:
```bash
ezkl create-evm-verifier
```
2. Deploy the generated contract to an Ethereum network using your preferred method (e.g., Hardhat, Truffle, or Remix).
3. Interact with the deployed contract:
```solidity
// Example Solidity contract integrating EZKL verification
contract MyContract {
    IEZKLVerifier public verifier;

    constructor(address _verifierAddress) {
        verifier = IEZKLVerifier(_verifierAddress);
    }

    function processProof(bytes calldata proof, uint256[] calldata publicInputs) external {
        bool isValid = verifier.verify(proof, publicInputs);
        require(isValid, "Invalid proof");
        
        // Continue with contract logic for valid proofs
        // ...
    }
}
```
4. Call the `processProof` function with the EZKL-generated proof and public inputs.

+++ EVM-Compatible Chains
The process is similar to Ethereum, so long as the chain has the XXX precompiles. Deploy the generated Solidity contract to your chosen EVM-compatible chain (e.g., Polygon, Binance Smart Chain, Avalanche) and interact with it using the chain's specific tools and SDKs.

Example for Polygon using Web3.js:
```javascript
const Web3 = require('web3');
const web3 = new Web3('https://polygon-rpc.com');

const verifierABI = [...]; // ABI of the verifier contract
const verifierAddress = '0x...'; // Address of the deployed verifier
const verifier = new web3.eth.Contract(verifierABI, verifierAddress);

const proof = '0x...'; // Your EZKL-generated proof
const publicInputs = [...]; // Your public inputs

verifier.methods.verify(proof, publicInputs).call()
    .then(isValid => {
        if (isValid) {
            console.log('Proof is valid');
            // Proceed with further on-chain actions
        } else {
            console.log('Proof is invalid');
        }
    })
    .catch(error => console.error('Verification failed:', error));
```
+++ Solana
Support for Solana verification is planned for future releases. Stay tuned for updates.
+++ Other
EZKL is actively working on supporting more blockchain platforms. Check the documentation or reach out to the community for the latest updates on supported chains.
+++

### Outputs
The verification result is a boolean value:
- `true`: The proof is valid. The prover has demonstrated knowledge of inputs that satisfy the circuit constraints.
- `false`: The proof is invalid. This could be due to an incorrect proof or tampered input data.

If you are attempting to read these results from a smart contract, you will need to decode the results. In Solidity, you might handle the result like this:
```solidity
IEZKLVerifier verifier = IEZKLVerifier(verifierAddress);
bool isValid = verifier.verify(proof, publicInputs);

if (isValid) {
    // Proof is valid - proceed with contract logic
    emit ProofVerified(msg.sender);
    // ... additional logic ...
} else {
    // Proof is invalid - handle accordingly
    revert("Invalid proof");
}
```