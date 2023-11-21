# Verify 

EZKL Verify enables in-browser evm verification of EZKL proofs.

---

```shell
# npm
npm install @ezkljs/verify

# yarn
yarn add @ezkljs/verify

# pnpm
pnpm add @ezkljs/verify
```

---

Open up the `@ezkljs/verify` codespace: [![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/zkonduit/inbrowser-evm-verify)
then run `pnpm run test` in the provided terminal to execute the jest tests on the library.


### Motivation

We would like the Solidity verifier to be canonical and usually all you ever need. For this, we need to be able to run that verifier in browser, using a lightweight EVM implementation, ethereumjs.

### Usage

`@ezkljs/verify` provides the `localEVMVerify` function which spins up an ephemeral EVM instance for executing the bytecode of a solidity verifier.

```typescript

import localEVMVerify from '@ezkljs/verify'

// Your proof and bytecode
const proof: Uint8Array
const bytecode: string

// Verify the proof
const result = await localEVMVerify(proof, bytecode)
console.log(result)  // true if the proof is valid
```

### localEVMVerify
    
```typescript
function localEVMVerify(
  proof: Uint8Array | Uint8ClampedArray,
  bytecode: string,
  evmVersion?: Hardfork,
): Promise<boolean>
```
Parameters:

- [proof] (Uint8Array | Uint8ClampedArray): The proof to be verified in serialized format.
- [bytecode] (string): The bytecode of the compiled Solidity verifier represented as a string.
- [evmVersion] (Hardfork [optional]): The Ethereum hardfork version target for the compiled bytecode. Default is Hardfork.London.

Return Value:

A `Promise` that resolves to a boolean indicating whether the verification succeeded.

### Example

Check out how the ezkljs verify library is used in the [ezkljs web app](http://localhost:3000/)
Link to the method used here in the example app: [handleEvmVerifyButton](https://github.com/zkonduit/ezkljs-engine/blob/main/app/Utils.tsx#L298)

```typescript
import localEVMVerify, { Hardfork } from '@ezkljs/verify'

export async function handleEvmVerifyButton<T extends FileMapping>(
  files: T,
  evmVersion: Hardfork
): Promise<VerifyResult> {
  const result = await convertFilesToFilesSer(files)

  const start = performance.now();  // Start the timer

  let output = await localEVMVerify(
    result['proof'],
    // Convert the serialized bytecode to a string using TextDecoder
    new TextDecoder().decode(result['bytecodeVerifier']),
    evmVersion
  )

  const end = performance.now();  // End the timer

  return {
    output: output,
    executionTime: end - start
  }
}

```

Output: 

```typescript
true
```

