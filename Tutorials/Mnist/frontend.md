---
order: 2
---

# MNIST Frontend Tutorial

This is part 3 of our tutorial on building the [e2e-minst](https://e2e-mnist.vercel.app) demo app; check out the [frontend code](https://github.com/zkonduit/e2e-mnist). 

# Overview

Armed with all the artifacts we need to prove in the browser and verify (on chain + browser), we need to build a frontend that can:

1. [Collect input data from the user (drawn digits)](#step-1-collecting-input-data)
2. [Initialize ezkljs engine module](#step-2-initializing-ezkljs-engine-module)
2. [Generate a witness](#step-3-generating-a-witness)
3. [Prove the witness statement against the ezkl model](#step-4-generating-a-proof)
4. [Verify the proof in the browser](#step-5-verifying-in-browser)
5. [Verify the proof on chain](#step-6-verifying-on-chain)

We will be leveraging the [ezkljs/engine](https://www.npmjs.com/package/@ezkljs/engine) npm package which contains the JS bindings for the ezkl library. Under the hood it uses [Web Assembly](https://webassembly.org/) to run the ezkl binaries in a browser context.

We imagine many other projects will follow a very similar flow, so will try our best to make this tutorial as general as possible so that it can be used as a reference for other ZKML projects. :)

## Step 1. Collecting Input Data

The first step is to collect the input data from the user. In our case, we want to collect a 28x28 pixel image of a hand drawn digit. Here is the Next.js code that renders a drawing board that we can use to faciltate such collection:

```tsx MNISTDraw.tsx
'use client'
import {
    Modal
} from 'flowbite-react'
import { useState, FC } from 'react';
import './MNIST.css';
import './App.css';

const size = 28;
interface IMNISTBoardProps {
    grid: number[][];
    onChange: (row: number, col: number) => void;
}

const MNISTBoard: FC<IMNISTBoardProps> = ({ grid, onChange }) => {
    const [mouseDown, setMouseDown] = useState(false);

    const GridSquare = (row: number, col: number) => {
        const handleChange = () => {
            if (mouseDown) {
                onChange(row, col);
            }
        };

        const handleMouseDown = () => {
            setMouseDown(true);
            onChange(row, col);
        };

        const handleMouseUp = () => {
            setMouseDown(false);
        };

        return (
            <div
                className={`square ${grid[row][col] ? 'on' : 'off'}`}
                onMouseEnter={handleChange}
                onMouseDown={handleMouseDown}
                onMouseUp={handleMouseUp}
            />
        );
    };

    const renderCol = (col: number) => {
        const mycol = [];
        for (let row = 0; row < size; row++) {
            mycol.push(<div key={`row-${row}`}>{GridSquare(row, col)}</div>);
        }
        return <div key={`col-${col}`}>{mycol}</div>;
    };

    const RenderGrid = () => {
        const mygrid = [];
        for (let i = 0; i < size; i++) {
            mygrid.push(renderCol(i));
        }
        return mygrid;
    };

    return (
        <div className="MNISTBoard">
            <div className="centerObject">
                <div className="grid">{RenderGrid()}</div>
            </div>
        </div>
    );
};

export function MNISTDraw() {
    const [grid, setGrid] = useState(Array(size).fill(null).map(() => Array(size).fill(0))); // initialize to a 28x28 array of 0's

    function handleSetSquare(myrow: number, mycol: number) {
        var newArray = [];
        for (var i = 0; i < grid.length; i++)
            newArray[i] = grid[i].slice();
        newArray[myrow][mycol] = 1;
        setGrid(newArray);
    }

    return (
        <>
            <div className="MNISTPage">
                <h1 className='text-2xl'>Draw and classify a digit</h1>
                <MNISTBoard grid={grid} onChange={(r, c) => handleSetSquare(r, c)} />
            </div>
        </>
    );
};
```

And the associated CSS for the drawing board:

```css MNIST.css
.MNISTBoard {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.centerObject {
  display: flex;
  justify-content: center;
  flex-direction: column;
}

.grid {
  display: flex;
  flex-direction: row
}

.square {
  float: center;
  position: relative;
  width: 1vw;
  height: 1vw;
  margin: 0%;
  background-color: #1E1E1E;
  overflow: hidden;
  border: 1px solid black;
}

.square.off {
  background-color: #FFFFFF;
}

.square.on {
  background-color: #1E1E1E;
}
```

## Step 2. Initializing ezkljs engine module

Next, we need to generate a witness from the input data. As mentioned earlier, we will need to utilize the ezkljs engine package for this. But before we can use any of the methods in the package, we need to initialize the web assembly module that will allow us to perform multi-threading to parrallelize the proving and thus improve perforamance. We do this by calling the default export of the engine module, passing in a [web assembly memory object](https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/Memory/Memory). By default, the memory allocated to the web worker is the maximum allowed for the wasm32-unknown-unkown target (65536 bytes) per WebAssembly page. Here is the code that does this:

```tsx ezkl.ts
// Initialize the engine and util imports in the SharedResourcesProvider component:
// Note: must import Uitls as a module instead of a component for engine to work (or else you will get 'self' is undefined wasm errors)
"use client";
import React, { createContext, useContext, useState, useEffect } from "react";
type Utils = typeof import("../lib/ezkl");
type Engine = typeof import("@ezkljs/engine/web/ezkl");

interface SharedResources {
  engine: Engine;
  utils: Utils;
}

const SharedResourcesContext = createContext<SharedResources | null>(null);
export const useSharedResources = (): SharedResources => {
  const context = useContext(SharedResourcesContext);
  if (!context) {
    throw new Error(
      "useSharedResources must be used within a SharedResourcesProvider"
    );
  }
  return context;
};

interface SharedResourcesProviderProps {
  children: React.ReactNode;
}

export const SharedResourcesProvider: React.FC<
  SharedResourcesProviderProps
> = ({ children }) => {
  const [engine, setEngine] = useState<Engine>();
  const [utils, setUtils] = useState<Utils>();

  useEffect(() => {
    async function initializeResources() {
      // Initialize the WASM module
      const engine = await import("@ezkljs/engine/web/ezkl.js");
      setEngine(engine);
      await (engine as any).default(
        undefined,
        new WebAssembly.Memory({ initial: 20, maximum: 65536, shared: true })
      );
      // For human readable wasm debug errors call this function
      engine.init_panic_hook();
      // Initialize the utils module
      const utils = await import("../lib/ezkl");
      setUtils(utils);
    }
    initializeResources();
  }, []);

  if (!engine || !utils) {
    return null;
  }

  return (
    <SharedResourcesContext.Provider value={{ engine, utils }}>
      {children}
    </SharedResourcesContext.Provider >
  );
};
```

We also need to create a utils module that will contain wrapper functions for the engine methods that we will use in the rest of the app.
This utils file will essentially handle proving and verifying by using the circuit artifacts that we generated in the previous section. We need to put these artifacts (`key.pk`, `key.vk`, `network.compiled`, `settings.json`, `kzg.srs`) in the `public` folder so that they can be accessed by the browser via the `fetch` method. Here is the code for the utils module:

```tsx ezkl.ts
import {
  prove,
  verify,
  genWitness
} from "@ezkljs/engine/web";

async function getDataBuffer(name: string): Promise<Uint8ClampedArray> {
  const response = await fetch(`/data/${name}`);
  const buffer = await response.arrayBuffer();
  return new Uint8ClampedArray(buffer);
}

export function handleFileDownload(fileName: string, buffer: Uint8Array) {
  // Create a blob from the buffer
  const blob = new Blob([buffer], { type: "application/octet-stream" });

  // Create an Object URL from the blob
  const url = window.URL.createObjectURL(blob);

  // Create an anchor element for the download
  const a = document.createElement("a");
  a.href = url;
  a.download = fileName;
  document.body.appendChild(a);

  // Trigger the download by simulating a click on the anchor element
  a.click();

  // Remove the anchor element after download
  document.body.removeChild(a);

  // Free up the Object URL
  window.URL.revokeObjectURL(url);
}

interface Uint8ArrayResult {
  output: Uint8Array;
  executionTime: number;
}

export async function handleGenProofButton(witness: Uint8ClampedArray) {
  const start = performance.now(); // Start the timer
  console.log("proof start")
  let output = prove(
    witness,
    await getDataBuffer("key.pk"),
    await getDataBuffer("network.compiled"),
    await getDataBuffer("kzg.srs")
  );

  const end = performance.now(); // End the timer

  return {
    output,
    executionTime: end - start,
  };
}

export async function handleGenWitnessButton(
  input: any[]
): Promise<Uint8ArrayResult> {
  const start = performance.now(); // Start the timer
  const formattedInput = {
    input_data: [input],
  };
  console.log("formattedInput", formattedInput);
  let output = genWitness(
    await getDataBuffer("network.compiled"),
    new Uint8ClampedArray(
      new TextEncoder().encode(JSON.stringify(formattedInput))
    )
  );

  const end = performance.now(); // End the timer

  return {
    output: output,
    executionTime: end - start,
  };
}

interface VerifyResult {
  output: boolean;
  executionTime: number;
}

export async function handleVerifyButton(
  proof: Uint8ClampedArray
): Promise<VerifyResult> {
  const start = performance.now(); // Start the timer

  let output = verify(
    proof,
    new Uint8ClampedArray(await getDataBuffer("key.vk")),
    new Uint8ClampedArray(await getDataBuffer("settings.json")),
    new Uint8ClampedArray(await getDataBuffer("kzg.srs"))
  );

  const end = performance.now(); // End the timer

  return {
    output: output,
    executionTime: end - start,
  };
}
```

For the wasm intialization to work, we need to wrap the entire app in a `SharedResourcesProvider` component. This component will provide the engine and utils modules to the rest of the app via the `useSharedResources` hook.

```tsx page.tsx
import type { Metadata } from "next";
import "./globals.css";
import { SharedResourcesProvider } from "@/providers/ezkl";

export const metadata: Metadata = {
  title: "Secret ID",
  description: "Keep your IDs safe and secure",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <SharedResourcesProvider>
          {children}
        </SharedResourcesProvider>
      </body>
    </html>
  );
}
```

## Step 3. Generating a Witness

Next, we need to generate a witness from the input data, as we will need this to generate a proof.
For this app, we will generate the witness as part of the proof generation process, so it will go in the `doProof` function that we will define in the code block below. 

- First we will need to convert the input data stored in the `grid` state into a 1D array of 784 (28 * 28) elements, as this is the size of the input layer of the model. After that we pass this flattened image tensor to the `handleGenWitnessButton` method from the utils module.

- Next we will need to deserialize the output of the `handleGenWitnessButton` method into a witness object. We do this by calling the `deserialize` method from the engine module on the `output`.

- Finally we will need to get the prediction from the witness object by calling the `getPrediction` method from the utils module. This function determines which output index has the highest value and returns that index as the predicted digit. Here is the code that does this:

```tsx MNISTDraw.tsx
export function MNISTDraw() {
    const [prediction, setPrediction] = useState<number>(-1);
    const [grid, setGrid] = useState(Array(size).fill(null).map(() => Array(size).fill(0))); // initialize to a 28x28 array of 0's

    const parseOutput = (output: any[][]) => {
        const convertedOutput = [];
        for (let item of output[0]) {
            const result = engine.vecU64ToInt(engine.serialize(item));
            const resultInt = engine.deserialize(result);
            convertedOutput.push(resultInt);
        }
        return convertedOutput;
    };

    const getPrediction = (output: any[][]) => {
        // Since the witness file contains raw quantized input and output field elements of the model 
        // (which we can't easily compare in JS b/c JS truncates all numbers above 2^53 - 1 to 2^53 - 1),
        // we need to convert them to integers. 
        const convertedOutput = parseOutput(output);
        const index = convertedOutput.indexOf(Math.max(...convertedOutput));
        return index;
    };

    async function doProof() {
        // get image from grid
        var imgTensor = Array(MNISTSIZE).fill(0);
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                imgTensor[i * size + j] = grid[i][j];
            }
        }
        // With image data, generate witness
        let witnessSer
        try {
            const { output, } = await utils.handleGenWitnessButton(
                imgTensor
            );
            witnessSer = output
            let witness = engine.deserialize(output);
            const prediction = getPrediction(witness.outputs);
            setPrediction(prediction);
            setPredictionDone(true);
        } catch (error) {
            console.error("An error occurred:", error);
        }
        // With witness, generate proof
    }

    function PredictionBlock() {
        return (
            <div className="predction color-white">
                <h2>Prediction</h2>
                {prediction}
            </div>
        );
    }

    function ProofButton() {
        return (
            <Button
                className={styles.button}
                text="Classify & Prove"
                loading={generatingProof}
                loadingText="Proving..."
                onClick={doProof}
            />
        );
    }

    function handleSetSquare(myrow: number, mycol: number) {
        var newArray = [];
        for (var i = 0; i < grid.length; i++)
            newArray[i] = grid[i].slice();
        newArray[myrow][mycol] = 1;
        setGrid(newArray);
    }

    return (
        <div className="MNISTPage">
            <h1 className='text-2xl'>Draw and classify a digit</h1>
            <MNISTBoard grid={grid} onChange={(r, c) => handleSetSquare(r, c)} />
            <div className="buttonPanel">
                <ProofButton />
                <VerifyInBrowserButton />
                <VerifyOnChainButton />
                <ResetButton />
            </div>
            {predictionDone && PredictionBlock()}
            {proofDone && ProofBlock()}
        </div>
    );
};
```

## Step 4. Generating a Proof

With the witness created, we can now generate a proof. We do this by calling the `handleGenProofButton` method from the utils module, passing in the witness as an argument. This method will return a serialzied proof object that we will need to deserialize and convert the `proof.proof` and `proof.instances` fields to a hex string and uint256 array respectively so that it can be passed to the solidity verifier later on. Here is the code that does this:

```tsx MNISTDraw.tsx
export function MNISTDraw() {
    const [prediction, setPrediction] = useState<number>(-1);
    const [predictionDone, setPredictionDone] = useState(false);
    const [proof, setProof] = useState<any | null>(null);
    const [proofDone, setProofDone] = useState(false);
    const [buffer, setBuffer] = useState<Uint8Array | null>(null);
    const [grid, setGrid] = useState(Array(size).fill(null).map(() => Array(size).fill(0))); // initialize to a 28x28 array of 0's

    const parseOutput = (output: any[][]) => {
        const convertedOutput = [];
        for (let item of output[0]) {
            const result = engine.vecU64ToInt(engine.serialize(item));
            const resultInt = engine.deserialize(result);
            convertedOutput.push(resultInt);
        }
        return convertedOutput;
    };

    const getPrediction = (output: any[][]) => {
        // Since the witness file contains raw quantized input and output field elements of the model 
        // (which we can't easily compare in JS b/c JS truncates all numbers above 2^53 - 1 to 2^53 - 1),
        // we need to convert them to integers. 
        const convertedOutput = parseOutput(output);
        const index = convertedOutput.indexOf(Math.max(...convertedOutput));
        return index;
    };

    async function doProof() {
        // get image from grid
        var imgTensor = Array(MNISTSIZE).fill(0);
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                imgTensor[i * size + j] = grid[i][j];
            }
        }
        // With image data, generate witness
        let witnessSer
        try {
            const { output, } = await utils.handleGenWitnessButton(
                imgTensor
            );
            witnessSer = output
            let witness = engine.deserialize(output);
            const prediction = getPrediction(witness.outputs);
            setPrediction(prediction);
            setPredictionDone(true);
        } catch (error) {
            console.error("An error occurred:", error);
        }
        // With witness, generate proof
        setGeneratingProof(true);
        const { output, executionTime } = await utils.handleGenProofButton(
            new Uint8ClampedArray(witnessSer!)
        );
        setBuffer(output);
        setGeneratingProof(false);
        const proof = engine.deserialize(output);

        console.log(`Proving time: ${executionTime}ms`);
        let instances = [];
        console.log("proof instances", proof.instances);
        for (let i = 0; i < proof.instances[0].length; i++) {
            let intSerialized = engine.serialize(proof.instances[0][i]);
            let intHex = engine.vecU64ToFelt(intSerialized);
            let int = BigInt(intHex).toString();
            instances.push(int);
        }
        const proofObj = {
            proof: engine.printProofHex(new Uint8ClampedArray(output)),
            instances
        }
        setProof(proofObj);
        console.log("proof", proof);
        setProofDone(true);
    }

    function ProofBlock() {
        return (
            <div className="proof">
                <Button
                    className="w-auto"
                    type='submit'
                    onClick={() => utils.handleFileDownload('test.pf', buffer!)}
                    text="Download Proof File"
                />
                <Button
                    className="w-auto"
                    onClick={() => props.setOpenModal('default')}
                    data-modal-target="witness-modal"
                    data-modal-toggle="witness-modal"
                    text="Show Proof"
                />
                <Modal
                    show={props.openModal === 'default'}
                    onClose={() => props.setOpenModal(undefined)}
                >
                    <Modal.Header>Proof: </Modal.Header>
                    <Modal.Body className="bg-black">
                        <div className='mt-4 p-4 bg-black-100 rounded'>
                            <pre className='blackspace-pre-wrap'>
                                {stringify(proof, null, 6)}
                            </pre>
                        </div>
                    </Modal.Body>
                </Modal>
            </div >
        );
    }

    function PredictionBlock() {
        return (
            <div className="predction color-white">
                <h2>Prediction</h2>
                {prediction}
            </div>ProofButton
        );
    }

    function ProofButton() {
        return (
            <Button
                className={styles.button}
                text="Classify & Prove"
                loading={generatingProof}
                loadingText="Proving..."
                onClick={doProof}
            />
        );
    }

    function resetImage() {
        var newArray = Array(size).fill(null).map(_ => Array(size).fill(0));
        setGrid(newArray);
        setPredictionDone(false);
        setProofDone(false);
    }

    function handleSetSquare(myrow: number, mycol: number) {
        var newArray = [];
        for (var i = 0; i < grid.length; i++)
            newArray[i] = grid[i].slice();
        newArray[myrow][mycol] = 1;
        setGrid(newArray);
    }

    return (
        <div className="MNISTPage">
            <h1 className='text-2xl'>Draw and classify a digit</h1>
            <MNISTBoard grid={grid} onChange={(r, c) => handleSetSquare(r, c)} />
            <div className="buttonPanel">
                <ProofButton />
                <ResetButton />
            </div>
            {predictionDone && PredictionBlock()}
            {proofDone && ProofBlock()}
        </div>
    );
};
```

## Step 5. Verifying in Browser

With the proof in hand, verifying in the browser is pretty straight forward: We can call the `handleVerifyButton` method from the utils module, passing in the proof as an argument. This method will return a boolean value indicating whether the proof is valid or not. Here is the code that does this:

```tsx MNISTDraw.tsx
export function MNISTDraw() {
    const { engine, utils } = useSharedResources();
    const [openModal, setOpenModal] = useState<string | undefined>();
    const props = { openModal, setOpenModal };
    const [prediction, setPrediction] = useState<number>(-1);
    const [predictionDone, setPredictionDone] = useState(false);
    const [proof, setProof] = useState<any | null>(null);
    const [proofDone, setProofDone] = useState(false);
    const [generatingProof, setGeneratingProof] = useState(false);
    const [buffer, setBuffer] = useState<Uint8Array | null>(null);
    const [isVerifiedInBrowser, setIsVerifiedInBrowser] = useState(false);
    const [verifyInBrowserDone, setVerifyInBrowserDone] = useState(false);
    const [grid, setGrid] = useState(Array(size).fill(null).map(() => Array(size).fill(0))); // initialize to a 28x28 array of 0's

    const parseOutput = (output: any[][]) => {
        const convertedOutput = [];
        for (let item of output[0]) {
            const result = engine.vecU64ToInt(engine.serialize(item));
            const resultInt = engine.deserialize(result);
            convertedOutput.push(resultInt);
        }
        return convertedOutput;
    };

    const getPrediction = (output: any[][]) => {
        // Since the witness file contains raw quantized input and output field elements of the model 
        // (which we can't easily compare in JS b/c JS truncates all numbers above 2^53 - 1 to 2^53 - 1),
        // we need to convert them to integers. 
        const convertedOutput = parseOutput(output);
        const index = convertedOutput.indexOf(Math.max(...convertedOutput));
        return index;
    };

    async function doProof() {
        // get image from grid
        var imgTensor = Array(MNISTSIZE).fill(0);
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                imgTensor[i * size + j] = grid[i][j];
            }
        }
        // With image data, generate witness
        let witnessSer
        try {
            const { output, } = await utils.handleGenWitnessButton(
                imgTensor
            );
            witnessSer = output
            let witness = engine.deserialize(output);
            const prediction = getPrediction(witness.outputs);
            setPrediction(prediction);
            setPredictionDone(true);
        } catch (error) {
            console.error("An error occurred:", error);
        }
        // With witness, generate proof
        setGeneratingProof(true);
        const { output, executionTime } = await utils.handleGenProofButton(
            new Uint8ClampedArray(witnessSer!)
        );
        setBuffer(output);
        setGeneratingProof(false);
        const proof = engine.deserialize(output);

        console.log(`Proving time: ${executionTime}ms`);
        let instances = [];
        console.log("proof instances", proof.instances);
        for (let i = 0; i < proof.instances[0].length; i++) {
            let intSerialized = engine.serialize(proof.instances[0][i]);
            let intHex = engine.vecU64ToFelt(intSerialized);
            let int = BigInt(intHex).toString();
            instances.push(int);
        }
        const proofObj = {
            proof: engine.printProofHex(new Uint8ClampedArray(output)),
            instances
        }
        setProof(proofObj);
        console.log("proof", proof);
        setProofDone(true);
    }

    async function doInBrowserVerify() {
        const { output, executionTime } = await utils.handleVerifyButton(new Uint8ClampedArray(buffer!));
        console.log(`Verifying in browser time: ${executionTime}ms`);
        setIsVerifiedInBrowser(output);
        setVerifyInBrowserDone(true);
    }

    function ProofBlock() {
        return (
            <div className="proof">
                <Button
                    className="w-auto"
                    type='submit'
                    onClick={() => utils.handleFileDownload('test.pf', buffer!)}
                    text="Download Proof File"
                />
                <Button
                    className="w-auto"
                    onClick={() => props.setOpenModal('default')}
                    data-modal-target="witness-modal"
                    data-modal-toggle="witness-modal"
                    text="Show Proof"
                />
                <Modal
                    show={props.openModal === 'default'}
                    onClose={() => props.setOpenModal(undefined)}
                >
                    <Modal.Header>Proof: </Modal.Header>
                    <Modal.Body className="bg-black">
                        <div className='mt-4 p-4 bg-black-100 rounded'>
                            <pre className='blackspace-pre-wrap'>
                                {stringify(proof, null, 6)}
                            </pre>
                        </div>
                    </Modal.Body>
                </Modal>
            </div >
        );
    }

    function PredictionBlock() {
        return (
            <div className="predction color-white">
                <h2>Prediction</h2>
                {prediction}
            </div>ProofButton
        );
    }

    function VerifyInBrowserBlock() {
        return (
            <div className="verify">
                <h1 className='text-2xl'>Verified in the browser: {JSON.stringify(isVerifiedInBrowser)}</h1>
            </div>
        );
    }

    function ProofButton() {
        return (
            <Button
                className={styles.button}
                text="Classify & Prove"
                loading={generatingProof}
                loadingText="Proving..."
                onClick={doProof}
            />
        );
    }

    function resetImage() {
        var newArray = Array(size).fill(null).map(_ => Array(size).fill(0));
        setGrid(newArray);
        setPredictionDone(false);
        setProofDone(false);        
        setVerifyInBrowserDone(false);
    }

    function handleSetSquare(myrow: number, mycol: number) {
        var newArray = [];
        for (var i = 0; i < grid.length; i++)
            newArray[i] = grid[i].slice();
        newArray[myrow][mycol] = 1;
        setGrid(newArray);
    }

    return (
        <div className="MNISTPage">
            <h1 className='text-2xl'>Draw and classify a digit</h1>
            <MNISTBoard grid={grid} onChange={(r, c) => handleSetSquare(r, c)} />
            <div className="buttonPanel">
                <ProofButton />
                <VerifyInBrowserButton />
                <ResetButton />
            </div>
            {predictionDone && PredictionBlock()}
            {proofDone && ProofBlock()}
            {verifyInBrowserDone && VerifyInBrowserBlock()}
        </div>
    );
};
```

## Step 6. Verifying on Chain

Last but not least, we need to be able to verify the proof on chain. For this we will need to call the `verifyProof` method on the `Halo2Verifier.sol` contract, passing in the `proof` and `instances` as arguments, returning `True` if the proof verifiers, reverting otherwise.

- First we will need to create a wagmi provider that will provide us read access to the contract deployed on the Mumbai network. Here is how that would look like: 

```tsx wagmi.tsx
"use client";

import { WagmiConfig, configureChains, createConfig } from "wagmi";
import { polygonMumbai } from 'viem/chains'
import { publicProvider } from 'wagmi/providers/public'

const { publicClient } = configureChains(
  [polygonMumbai],
  [publicProvider()]
);

const config = createConfig({
  publicClient
});

export default function WagmiProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return <WagmiConfig config={config}>{children}</WagmiConfig>;
}

```

- Next we will wrap this provider around the app so that we can use the `useWagmi` hook to access the contract. Here is how that would look like:

```tsx page.tsx
import type { Metadata } from "next";
import "./globals.css";
import WagmiProvider from "@/providers/wagmi";
import { SharedResourcesProvider } from "@/providers/ezkl";

export const metadata: Metadata = {
  title: "Secret ID",
  description: "Keep your IDs safe and secure",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <SharedResourcesProvider>
          <WagmiProvider>{children}</WagmiProvider>
        </SharedResourcesProvider>
      </body>
    </html>
  );
}

``` 

- Finally we will instantiate the verifier contract and call the `verifyProof` method on it, passing in the `proof` and `instances` as arguments. Here is the code that does this:

```tsx MNISTDraw.tsx
'use client'
import {
    Modal
} from 'flowbite-react'
import { useState, FC } from 'react';
import './MNIST.css';
import './App.css';
import { useSharedResources } from "@/providers/ezkl";
import { Button } from "@/components/button/Button";
import styles from "../../app/styles.module.scss";
import { stringify } from "json-bigint";
import { getContract } from 'wagmi/actions';
import { publicProvider } from 'wagmi/providers/public';
const size = 28;
const MNISTSIZE = 784;

const address = "0xAB5d009d3dcbdCB8432e53cde6BeF4A38Db7Bdc4";

const abi = [
    {
        "inputs": [
            {
                "internalType": "bytes",
                "name": "proof",
                "type": "bytes"
            },
            {
                "internalType": "uint256[]",
                "name": "instances",
                "type": "uint256[]"
            }
        ],
        "name": "verifyProof",
        "outputs": [
            {
                "internalType": "bool",
                "name": "",
                "type": "bool"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]
export function MNISTDraw() {
    const { engine, utils } = useSharedResources();
    const [openModal, setOpenModal] = useState<string | undefined>();
    const props = { openModal, setOpenModal };
    const [prediction, setPrediction] = useState<number>(-1);
    const [proof, setProof] = useState<any | null>(null);
    const [buffer, setBuffer] = useState<Uint8Array | null>(null); // proof file buffer
    const [generatingProof, setGeneratingProof] = useState(false);
    // On chain verification states
    const [generatingOnChainVerification, setGeneratingOnChainVerification] = useState(false);
    const [isVerifiedOnChain, setIsVerifiedOnChain] = useState(false);
    const [verifyOnChainDone, setVerifyOnChainDone] = useState(false);
    // In browser verification states
    const [generatingInBrowserVerification, setGeneratingInBrowserVerification] = useState(false);
    const [isVerifiedInBrowser, setIsVerifiedInBrowser] = useState(false);
    const [verifyInBrowserDone, setVerifyInBrowserDone] = useState(false);

    const [proofDone, setProofDone] = useState(false);
    const [predictionDone, setPredictionDone] = useState(false);
    const [grid, setGrid] = useState(Array(size).fill(null).map(() => Array(size).fill(0))); // initialize to a 28x28 array of 0's

    const parseOutput = (output: any[][]) => {
        const convertedOutput = [];
        for (let item of output[0]) {
            const result = engine.vecU64ToInt(engine.serialize(item));
            const resultInt = engine.deserialize(result);
            convertedOutput.push(resultInt);
        }
        return convertedOutput;
    };

    const getPrediction = (output: any[][]) => {
        const convertedOutput = parseOutput(output);
        console.log("convertedOutput", convertedOutput);
        const index = convertedOutput.indexOf(Math.max(...convertedOutput));
        return index;
    };

    async function doProof() {
        // get image from grid
        var imgTensor = Array(MNISTSIZE).fill(0);
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                imgTensor[i * size + j] = grid[i][j];
            }
        }
        let witnessSer
        try {
            const { output, } = await utils.handleGenWitnessButton(
                imgTensor
            );
            witnessSer = output
            let witness = engine.deserialize(output);
            const prediction = getPrediction(witness.outputs);
            setPrediction(prediction);
            setPredictionDone(true);
        } catch (error) {
            console.error("An error occurred:", error);
        }
        setGeneratingProof(true);
        const { output, executionTime } = await utils.handleGenProofButton(
            new Uint8ClampedArray(witnessSer!)
        );
        setBuffer(output);
        setGeneratingProof(false);
        const proof = engine.deserialize(output);

        console.log(`Proving time: ${executionTime}ms`);
        let instances = [];
        console.log("proof instances", proof.instances);
        for (let i = 0; i < proof.instances[0].length; i++) {
            let intSerialized = engine.serialize(proof.instances[0][i]);
            let intHex = engine.vecU64ToFelt(intSerialized);
            let int = BigInt(intHex).toString();
            instances.push(int);
        }
        const proofObj = {
            proof: engine.printProofHex(new Uint8ClampedArray(output)),
            instances
        }
        setProof(proofObj);
        console.log("proof", proof);
        setProofDone(true);
    }

    async function doInBrowserVerify() {
        const { output, executionTime } = await utils.handleVerifyButton(new Uint8ClampedArray(buffer!));
        console.log(`Verifying in browser time: ${executionTime}ms`);
        setIsVerifiedInBrowser(output);
        setVerifyInBrowserDone(true);
    }

    async function doOnChainVerify() {
        // Replace with your contract's ABI and address

        const provider = publicProvider();

        // Instantiate the contract using wagmi's getContract hook
        const contract = getContract({
            address: address,
            abi: abi,
            walletClient: provider,
            chainId: 80001
        });

        try {
            let result: boolean;
            result = await contract.read.verifyProof([
                `0x${proof.proof}`,
                proof.instances
            ]) as boolean

            setIsVerifiedOnChain(result);
            setVerifyOnChainDone(true);
        } catch (error) {
            // window error popup
            window.alert(`Verification failed with error: ${error}`);
            console.log(`Verification failed with error: ${error}`)
            setVerifyOnChainDone(false);
        }
    }

    function resetImage() {
        var newArray = Array(size).fill(null).map(_ => Array(size).fill(0));
        setGrid(newArray);
        setPredictionDone(false);
        setProofDone(false);
        setVerifyInBrowserDone(false);
        setVerifyOnChainDone(false);
    }

    function handleSetSquare(myrow: number, mycol: number) {
        var newArray = [];
        for (var i = 0; i < grid.length; i++)
            newArray[i] = grid[i].slice();
        newArray[myrow][mycol] = 1;
        setGrid(newArray);
    }

    function ProofButton() {
        return (
            <Button
                className={styles.button}
                text="Classify & Prove"
                loading={generatingProof}
                loadingText="Proving..."
                onClick={doProof}
            />
        );
    }

    function VerifyInBrowserButton() {
        return (
            <Button
                className={styles.button}
                text="Verify in Browser"
                disabled={!proofDone}
                loading={generatingInBrowserVerification}
                loadingText="Verifying..."
                onClick={doInBrowserVerify}
            />
        );
    }

    function VerifyOnChainButton() {
        return (
            <Button
                className={styles.button}
                text="Verify on chain"
                disabled={!proofDone}
                loading={generatingOnChainVerification}
                loadingText="Verifying..."
                onClick={doOnChainVerify}
            />
        );
    }

    function ResetButton() {
        return (
            <Button
                className={styles.button}
                text="Reset"
                onClick={resetImage}
            />
        );
    }

    function ProofBlock() {
        return (
            <div className="proof">
                <Button
                    className="w-auto"
                    type='submit'
                    onClick={() => utils.handleFileDownload('test.pf', buffer!)}
                    text="Download Proof File"
                />
                <Button
                    className="w-auto"
                    onClick={() => props.setOpenModal('default')}
                    data-modal-target="witness-modal"
                    data-modal-toggle="witness-modal"
                    text="Show Proof"
                />
                <Modal
                    show={props.openModal === 'default'}
                    onClose={() => props.setOpenModal(undefined)}
                >
                    <Modal.Header>Proof: </Modal.Header>
                    <Modal.Body className="bg-black">
                        <div className='mt-4 p-4 bg-black-100 rounded'>
                            <pre className='blackspace-pre-wrap'>
                                {stringify(proof, null, 6)}
                            </pre>
                        </div>
                    </Modal.Body>
                </Modal>
            </div >
        );
    }

    function PredictionBlock() {
        return (
            <div className="predction color-white">
                <h2>Prediction</h2>
                {prediction}
            </div>
        );
    }

    function VerifyInBrowserBlock() {
        return (
            <div className="verify">
                <h1 className='text-2xl'>Verified in the browser: {JSON.stringify(isVerifiedInBrowser)}</h1>
            </div>
        );
    }

    function VerifyOnChainBlock() {
        return (
            <div className="verify">
                <h1 className='text-2xl'>
                    Verified by on chain smart { }
                    <a href={`https://mumbai.polygonscan.com/address/${address}`} target="_blank" rel="noopener noreferrer" style={{ textDecoration: 'underline' }}>
                        contract
                    </a>: {JSON.stringify(isVerifiedOnChain)}
                </h1>
            </div>

        );
    }

    return (
        <div className="MNISTPage">
            <h1 className='text-2xl'>Draw and classify a digit</h1>
            <MNISTBoard grid={grid} onChange={(r, c) => handleSetSquare(r, c)} />
            <div className="buttonPanel">
                <ProofButton />
                <VerifyInBrowserButton />
                <VerifyOnChainButton />
                <ResetButton />
            </div>
            {predictionDone && PredictionBlock()}
            {proofDone && ProofBlock()}
            {verifyInBrowserDone && VerifyInBrowserBlock()}
            {verifyOnChainDone && VerifyOnChainBlock()}
        </div>
    );
};
```



