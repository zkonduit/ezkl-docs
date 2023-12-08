---
order: 2
---

# MNIST Frontend Tutorial

This is part 4 of our tutorial on building the [e2e-minst](https://e2e-mnist.vercel.app) demo app; check out the [frontend code](https://github.com/zkonduit/e2e-mnist). 

# Overview

Armed with the artifacts we need to prove via the hub and verify on-chain, we need to build a frontend that can:

1. [Collect input data from the user (drawn digits)](#step-1-collecting-input-data)
2. [Generate proofs using hub](#step-2-generating-proofs-using-hub)
3. [Submit digit for on-chain verification](#step-3-verifying-on-chain)

We imagine many other projects that use EZKL Hub will follow a very similar flow for their front ends, so will try our best to make this tutorial as general as possible so that it can be used as a reference for other ZKML projects. :)

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

## Step 2. Generating Proofs Using Hub

Next, we need to generate a proof using the EZKL Hub. We do this by calling the `initiateProof` and `getProof` methods from the hub module. Run `pnpm i @ezkljs/hub` to install the module.  The `initiateProof` method takes in the artifact id, input data, and hub url as arguments and returns an id that we can use to get the proof later on. Copy the artifact id for this project from the hub dashboard and paste it into the `artifactId` variable. The `getProof` method takes in the id and hub url as arguments and returns the proof.

Once we get the proof, we need to parse the `instances` field of the proof to get the prediction. We do this by converting the `instances` field to a BigInt array and then finding the index of the max value of the array. To do this we convert each field element to an integer using the same logic in the intToFieldElement function in the solidity verifier contract. We then find the index of the max value of the array. This index is the predicted digit. Here is the code that does this:

```tsx ezkl.ts
import { hub } from "@ezkljs/hub";
import { useState } from "react";

export function MNISTDraw() {
    const [prediction, setPrediction] = useState<number>(-1);
    const [grid, setGrid] = useState(Array(size).fill(null).map(() => Array(size).fill(0))); // initialize to a 28x28 array of 0's
    const [generatingProof, setGeneratingProof] = useState(false);
    const [proofDone, setProofDone] = useState(false);
    const [proof, setProof] = useState<any>(null);

    async function doProof() {
        // get image from grid
        let imgTensor: number[] = Array(MNISTSIZE).fill(0)
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                imgTensor[i * size + j] = grid[i][j]
            }
        }

        const inputFile = JSON.stringify({ input_data: [imgTensor] })

        const url = 'https://hub-staging.ezkl.xyz/graphql'

        const artifactId = "insert-your-artifact-id-here"

        setGeneratingProof(true)
        try {
            const initiateProofResp = await hub.initiateProof({
                artifactId,
                inputFile,
                url,
            })

            let { status } = initiateProofResp
            const { id } = initiateProofResp

            let getProofResp
            while (status !== 'SUCCESS') {
                getProofResp = await hub.getProof({
                    id,
                    url,
                })

                status = getProofResp.status

                if (status === 'SUCCESS') {
                    break
                }
                await new Promise((resolve) => setTimeout(resolve, 2_000))
            }
            console.log('getProofResp', getProofResp?.instances)

            const p = BigInt(
                '0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001'
            )
            setProof(getProofResp)
            console.log('proof', JSON.stringify(getProofResp?.instances))
            console.log("proof", getProofResp?.proof)

            // convert each field element to an integer
            const results = getProofResp?.instances?.map((instance) => {
                const bigInst = BigInt(instance)
                // is negative
                if (bigInst > BigInt(2) ** BigInt(127) - BigInt(1)) {
                    return bigInst - p
                } else {
                    return bigInst
                }
            })

            console.log('results', results)

            if (!results || results.length === 0) {
                throw new Error('Array is empty')
            }

            // find the the index of the max value of the results array which contains BigInts
            // const index = results?.indexOf(results.reduce((a, b) => (a > b ? a : b)))
            if (results.length === 0) {
                throw new Error('Array is empty')
            }

            let maxIndex = 0
            let maxValue = results[0] // Assuming results is a non-empty array of BigInts

            for (let i = 1; i < results.length; i++) {
                if (results[i] > maxValue) {
                    maxValue = results[i]
                    maxIndex = i
                }
            }
            setPrediction(maxIndex)
            setProofDone(true)
            // console.log('index', index)
        } catch (error) {
            console.log('error', error)
        }
        setGeneratingProof(false)
    }
```

## Step 3. Verifying on Chain

Last but not least, we need to be able to verify the proof on chain. For this we will need to call the `submitDigit` method on the `MnistClan.sol` contract, passing in the `proof` and `instances` as arguments.

- First we will need to create a wagmi provider and a rainbow wallet that will provide us read access to the contract deployed on the Optimism Goerli network and a wallet connection respectively. 

Here is how that would look like: 

```tsx wagmi.tsx
"use client";

import '@rainbow-me/rainbowkit/styles.css';
import {
  getDefaultWallets,
  RainbowKitProvider
} from '@rainbow-me/rainbowkit';
import { configureChains, createConfig, WagmiConfig } from 'wagmi';
import { alchemyProvider } from 'wagmi/providers/alchemy';
import { publicProvider } from 'wagmi/providers/public';

import { optimismGoerli } from "wagmi/chains";

const { chains, publicClient } = configureChains(
  [optimismGoerli],
  [
    alchemyProvider({
      apiKey: process.env.NEXT_PUBLIC_ALCHEMY_KEY!,
    }),
    publicProvider()
  ]
);

const { connectors } = getDefaultWallets({
  appName: 'My RainbowKit App',
  projectId: 'YOUR_PROJECT_ID',
  chains
});

const wagmiConfig = createConfig({
  autoConnect: true,
  connectors,
  publicClient
})

export default function WagmiProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <WagmiConfig config={wagmiConfig}>
      <RainbowKitProvider chains={chains}>
        {children}
      </RainbowKitProvider>
    </WagmiConfig>
  )
}

```

- Next we will wrap this provider around the app so that we can use the `useWagmi` hook to access the contract. Here is how that would look like:

```tsx page.tsx
import type { Metadata } from "next";
import "./globals.css";
import WagmiProvider from "@/providers/wagmi";

export const metadata: Metadata = {
  title: "MNIST Clan",
  description: "Submit a ZKML proof of your classified handrawn digit to join the clan.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <WagmiProvider>{children}</WagmiProvider>
      </body>
    </html>
  );
}
``` 

- Next we will need to create a couple of json files that store the address and ABI of each contract that we deployed and then import that info into our app.

- You can get the ABI from remix by clicking on the solidity compiler icon and then the ABI button at the bottom under "Compilation Details". 

Here is how that would look like:

```json Halo2Verifier.json
{
    "address": "0x5cAC7Bf60B6936deE485643A2c2a708458C9f225",
    "abi": [
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
}
```

```json MnistClan.json
{
    "address": "0xf5cDCD333E3Fd09929BAcEa32c2c1E3A5A746d45",
    "abi": [
        {
            "inputs": [
                {
                    "internalType": "contract Verifier",
                    "name": "_verifier",
                    "type": "address"
                }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                }
            ],
            "name": "clan",
            "outputs": [
                {
                    "internalType": "uint8",
                    "name": "",
                    "type": "uint8"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "uint8",
                    "name": "",
                    "type": "uint8"
                }
            ],
            "name": "counts",
            "outputs": [
                {
                    "internalType": "uint256",
                    "name": "",
                    "type": "uint256"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
                {
                    "internalType": "address",
                    "name": "",
                    "type": "address"
                }
            ],
            "name": "entered",
            "outputs": [
                {
                    "internalType": "bool",
                    "name": "",
                    "type": "bool"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "getCounts",
            "outputs": [
                {
                    "internalType": "uint256[10]",
                    "name": "",
                    "type": "uint256[10]"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        },
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
            "name": "submitDigit",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "verifier",
            "outputs": [
                {
                    "internalType": "contract Verifier",
                    "name": "",
                    "type": "address"
                }
            ],
            "stateMutability": "view",
            "type": "function"
        }
    ]
}
```

- Finally we will instantiate both the mnist clan and verifier contracts. If the user hasn't submited a digit already, the proofs of digit recognition will get sent to the `MnistClan.sol` contract when the `SubmitMnistDigitButton` is clicked. If the user has already submited a digit, the `VerifyOnChainButton` will be appear instead and will merely statically call the `verifyProof` method on the `Halo2Verifier.sol` contract. 

- We will also display the counts of each digit submitted by all users in the form of a bar graph, as well as the user's clan and rank within the clan. 

Here is the code that does this:

```tsx MNISTDraw.tsx
'use client'
import { Modal } from 'flowbite-react'
import { useState, useEffect, FC } from 'react'
import './MNIST.css'
import './App.css'
import { Button } from '@/components/button/Button'
import styles from '../../app/styles.module.scss'
import { stringify } from 'json-bigint'
import { getContract } from 'wagmi/actions'
import { publicProvider } from 'wagmi/providers/public'
import { useAccount, usePrepareContractWrite, useContractWrite, useWaitForTransaction } from 'wagmi'
import { ConnectButton } from '@rainbow-me/rainbowkit';
import BarGraph from '../bargraph/BarGraph'; // Adjust the path as necessary
import hub from '@ezkljs/hub'
import MNIST from '../../contract_data/MnistClan.json'
import Verifier from '../../contract_data/Halo2Verifier.json'
const size = 28
const MNISTSIZE = 784

export function MNISTDraw() {
    const [openModal, setOpenModal] = useState<string | undefined>()
    const props = { openModal, setOpenModal }
    const [prediction, setPrediction] = useState<number>(-1)
    const [proof, setProof] = useState<any | null>(null)
    const [generatingProof, setGeneratingProof] = useState(false)
    const [counts, setCounts] = useState<number[] | null>(null)
    const [clan, setClan] = useState<number | null>(null)
    const [clanRank, setClanRank] = useState<number | null>(null)
    const [verifyResult, setVerifyResult] = useState<boolean | null>(null)

    const [proofDone, setProofDone] = useState(false)
    const [grid, setGrid] = useState<number[][]>(
        Array(size)
            .fill(null)
            .map(() => Array(size).fill(0))
    ) // initialize to a 28x28 array of 0's

    const { address, isConnected } = useAccount()

    const {
        config
    } = usePrepareContractWrite({
        address: MNIST.address as `0x${string}`,
        abi: MNIST.abi,
        functionName: 'submitDigit',
        args: [
            proof?.proof,
            proof?.instances
        ],
        enabled: true,
    })
    const { data, error, isError, write } = useContractWrite(config)
    const { isLoading, isSuccess } = useWaitForTransaction({
        hash: data?.hash,
    })

    const provider = publicProvider()

    // Instantiate the contract using wagmi's getContract hook
    const contract = getContract({
        address: MNIST.address as `0x${string}`,
        abi: MNIST.abi,
        walletClient: publicProvider(),
        chainId: 420,
    })

    async function getAccountClanInfo() {
        let entry = await contract.read.entered([address]) as boolean
        let clan = await contract.read.clan([address]) as number
        setClan(entry ? clan : null)
        console.log('entry', entry)
        console.log('clan', clan)
        let counts = await contract.read.getCounts() as number[]
        // convert BigInt to number
        counts = counts.map((count) => Number(count))
        // determine clan rank
        setCounts(counts)
        if (!entry) {
            return
        }
        let rank = 1
        for (let i = 0; i < counts.length; i++) {
            if (counts[i] > counts[clan]) {
                rank++
            }
        }
        setClanRank(rank)
        console.log('counts', counts)
    }

    useEffect(() => {
        (async () => {
            if (isConnected && (!clan || isSuccess)) {
                getAccountClanInfo()
            }
            if (!isConnected && clan) {
                setClan(null)
                setCounts(null)
            }
        })()
    }, [isConnected, isSuccess, address])

    // Reload clan info when account changes
    useEffect(() => {
        if (isConnected) {
            setClan(null)
            setCounts(null)
        }
    }, [address, isConnected]);

    function ShowClanResultsBlock() {
        if (!counts) {
            return
        }
        return (
            <div>
                <div className="MNISTClanChart">
                    <div className="chart-container">
                        <BarGraph data={counts} />
                    </div>
                </div>

            </div>
        )
    }


    async function doProof() {
        // get image from grid
        let imgTensor: number[] = Array(MNISTSIZE).fill(0)
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                imgTensor[i * size + j] = grid[i][j]
            }
        }

        const inputFile = JSON.stringify({ input_data: [imgTensor] })

        const url = 'https://hub-staging.ezkl.xyz/graphql'

        const artifactId = "d079f79d-a902-43e6-a3a5-b22b0efdbc6a"

        setGeneratingProof(true)
        try {
            const initiateProofResp = await hub.initiateProof({
                artifactId,
                inputFile,
                url,
            })
            // console.log('initiateProofResp', initiateProofResp)

            let { status } = initiateProofResp
            const { id } = initiateProofResp

            let getProofResp
            while (status !== 'SUCCESS') {
                getProofResp = await hub.getProof({
                    id,
                    url,
                })

                status = getProofResp.status

                if (status === 'SUCCESS') {
                    break
                }
                await new Promise((resolve) => setTimeout(resolve, 2_000))
            }
            console.log('getProofResp', getProofResp?.instances)

            const p = BigInt(
                '0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001'
            )
            setProof(getProofResp)
            console.log('proof', JSON.stringify(getProofResp?.instances))
            console.log("proof", getProofResp?.proof)
            const results = getProofResp?.instances?.map((instance) => {
                const bigInst = BigInt(instance)
                // is negative
                if (bigInst > BigInt(2) ** BigInt(128) - BigInt(1)) {
                    return bigInst - p
                } else {
                    return bigInst
                }
            })

            console.log('results', results)

            if (!results || results.length === 0) {
                throw new Error('Array is empty')
            }

            // find the the index of the max value of the results array which contains BigInts
            // const index = results?.indexOf(results.reduce((a, b) => (a > b ? a : b)))
            if (results.length === 0) {
                throw new Error('Array is empty')
            }

            let maxIndex = 0
            let maxValue = results[0] // Assuming results is a non-empty array of BigInts

            for (let i = 1; i < results.length; i++) {
                if (results[i] > maxValue) {
                    maxValue = results[i]
                    maxIndex = i
                }
            }
            setPrediction(maxIndex)
            setProofDone(true)
            // console.log('index', index)
        } catch (error) {
            console.log('error', error)
        }
        setGeneratingProof(false)
    }


    async function doOnChainVerify() {

        let verifierContract = getContract({
            address: Verifier.address as `0x${string}`,
            abi: Verifier.abi,
            walletClient: provider,
            chainId: 420,
        })

        let result = await verifierContract.read.verifyProof([proof?.proof, proof?.instances]) as boolean
        setVerifyResult(result);
    }

    async function doSubmitMnistDigit() {
        if (!write) { return }
        write()
    }

    function resetImage() {
        var newArray = Array(size)
            .fill(null)
            .map((_) => Array(size).fill(0))
        setGrid(newArray)
        setProofDone(false)
        setVerifyResult(null)
    }

    function handleSetSquare(myrow: number, mycol: number) {
        var newArray = []
        for (var i = 0; i < grid.length; i++) newArray[i] = grid[i].slice()
        newArray[myrow][mycol] = 1
        setGrid(newArray)
    }

    function ProofButton() {
        return (
            <Button
                className={styles.button}
                text='Classify & Prove'
                loading={generatingProof}
                loadingText='Proving...'
                onClick={doProof}
            />
        )
    }

    function VerifyOnChainButton() {
        return (
            <Button
                className={styles.button}
                text='Verify On Chain'
                disabled={!proofDone}
                loading={isLoading}
                loadingText='Verifying...'
                onClick={doOnChainVerify}
            />
        )
    }

    function SubmitMnistDigitButton() {
        return (
            <Button
                className={styles.button}
                text='Submit Mnist Digit'
                disabled={!proofDone || !write || isLoading}
                loading={isLoading}
                loadingText='Verifying...'
                onClick={doSubmitMnistDigit}
            />
        )
    }

    function ResetButton() {
        return (
            <Button className={styles.button} text='Reset' onClick={resetImage} />
        )
    }

    function ProofBlock() {
        return (
            <div className='proof'>
                <Button
                    className='w-auto'
                    onClick={() => props.setOpenModal('default')}
                    data-modal-target='witness-modal'
                    data-modal-toggle='witness-modal'
                    text='Show Proof'
                />
                <Modal
                    show={props.openModal === 'default'}
                    onClose={() => props.setOpenModal(undefined)}
                >
                    <Modal.Header>Proof: </Modal.Header>
                    <Modal.Body className='bg-black'>
                        <div className='mt-4 p-4 bg-black-100 rounded'>
                            <pre className='blackspace-pre-wrap'>
                                {stringify(proof, null, 6)}
                            </pre>
                        </div>
                    </Modal.Body>
                </Modal>
            </div>
        )
    }

    function PredictionBlock() {
        return (
            <div className='predction color-white'>
                <h1>Prediction</h1>
                {prediction}
            </div>
        )
    }

    function VerifyOnChainBlock() {
        return (
            <div className='verify'>
                <h1 className='text-2xl'>
                    Verified on chain: { }
                    <a
                        href={`https://goerli-optimism.etherscan.io/address/${Verifier.address}#code`}
                        target='_blank'
                        rel='noopener noreferrer'
                        style={{ textDecoration: 'underline' }}
                    >
                        {Verifier.address}
                    </a>
                </h1>
            </div>
        )
    }

    if (proofDone && isError) {
        window.alert(`Transaction failed on MnistClan contract:${error?.message}`)
    }

    return (
        <div className='MNISTPage'>
            <h1 className='text-2xl'>Draw and classify a digit</h1>
            <MNISTBoard grid={grid} onChange={(r, c) => handleSetSquare(r, c)} />
            <div className='flex justify-center pt-7'>
                <ConnectButton />
            </div>
            {clan && <h1 className='text-2xl pt-7'>Your MNIST Clan: {clan} </h1>}
            {clan && <h1 className='text-2xl'>Your Clan Rank: {clanRank} </h1>}
            <div className='buttonPanel'>
                <ProofButton />
                {clan ? <VerifyOnChainButton /> : <SubmitMnistDigitButton />}
                <ResetButton />
            </div>
            {proofDone && PredictionBlock()}
            {proofDone && ProofBlock()}
            {(isSuccess || !(verifyResult == null)) && VerifyOnChainBlock()}
            {clan && ShowClanResultsBlock()}
        </div>
    )
}
```



