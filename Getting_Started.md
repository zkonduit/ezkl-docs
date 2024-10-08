---
icon: code-square
order: 93
---
![](../assets/cli.png) 

## Tutorial 👾

You can easily create an `.onnx` file using `pytorch`. For samples of Onnx files see [here](https://github.com/onnx/models). To see how to generate Onnx files using python, check out <a href="https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/" target="_blank"> the notebooks.</a>. You'll also need an `input.json` file with sample inputs and outputs of your model.

Sample onnx files are also available in <a href="https://github.com/zkonduit/ezkl/blob/main/examples/onnx/" target="_blank"> the repo </a>.

#### Initializing the project
To generate a proof on one of the examples, first install `ezkl` 
[!ref](/installing)

Put a model file (`network.onnx`) and input file (`input.json`) into your working directory, e.g. with something like:
```bash
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/network.onnx ./
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/input.json ./

```

(You can get these specific files from [this directory](https://github.com/zkonduit/ezkl/tree/main/examples/onnx/4l_relu_conv_fc/).)

To display `ezkl`'s understanding of the model in the CLI, run:

```bash
ezkl table -M network.onnx
```

You can always check the options available for a command by typing the command with `--help`. For example, `ezkl table --help` will show you the options available for the `table` command. This will provide you with the most up-to-date information on a given command's usage and the cli spec. 

```bash
# list all available commands
ezkl --help

# list all available options for COMMAND
ezkl COMMAND --help
```

### Proving Backend (Lilith)

Running ZKML proofs can be computationally expensive. We've made the process easier by providing a backend service that can run the proofs for you.

If you're interested in using the Lilith backend, you can register your interest [here](https://ei40vx5x6j0.typeform.com/to/sFv1oxvb).


#### Setting circuit parameters

+++ CLI
Our circuit is configured with the `settings.json` file. This is created with the `gen-settings` command. 
```bash
ezkl gen-settings
```
This will produce a `settings.json` file you can use for your circuit. However, you can fine-tune your circuit to optimize for accuracy or CPU/memory usage with the `calibrate-settings` command:
```bash
ezkl calibrate-settings --target resources
```
In this example, we set the `--target` to **"resources"** so that we can optimize for CPU and memory usage. The other option is **"accuracy"**, which optimizes for accuracy given the fixed point representation of the input model. Our circuit parameters are generated, then saved to `settings.json`. 

Download the appropriate SRS:
```bash
ezkl get-srs
```
+++ Python
From the `network.onnx` onnx file, we will create a `settings.json` file that uses the `py_run_args` file to specify the visibility of the inputs, outputs and paramaters of the model. 

Once we have created the settings file, we can calibrate it using the `ezkl.calibrate_settings` command to optimize for either `accuracy` or `resources` using the input to the model (`input.json`), and the model itself (`network.onnx`). The accurary target will optimize the circuit for accuracy given the fixed point representation of the input model. The resources target will optimize the circuit for CPU and memory usage.

Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet. 

```python
py_run_args = ezkl.PyRunArgs()
py_run_args.input_visibility = "public"
py_run_args.output_visibility = "public"
py_run_args.param_visibility = "fixed" # "fixed" for params means that the committed to params are used for all proofs

res = ezkl.gen_settings()
assert res == True

res = await ezkl.calibrate_settings(target="resources")
assert res == True
```
+++ JS
For performance reasons, you can only generate settings using the hub, python and cli environments. Stay tuned for updates!
+++


#### Compiling the model

+++ CLI
From the onnx file, we will create a `.ezkl` file that uses the settings to convert the onnx model to a format ready for proving.

```bash
ezkl compile-circuit -M network.onnx -S settings.json --compiled-circuit network.ezkl
```
+++ Python
From the `network.onnx` onnx file, we will create a `network.compiled` file that uses the `settings.json` file to convert the onnx model to a format ready for proving. Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet.

```python
model_path = os.path.join('network.onnx')
compiled_model_path = os.path.join('network.compiled')
settings_path = os.path.join('settings.json')
data_path = os.path.join('input.json')

py_run_args = ezkl.PyRunArgs()
py_run_args.input_visibility = "public"
py_run_args.output_visibility = "public"
py_run_args.param_visibility = "fixed" # "fixed" for params means that the committed to params are used for all proofs

res = ezkl.gen_settings(model_path, settings_path, py_run_args=py_run_args)
assert res == True

res = await ezkl.calibrate_settings(data_path, model_path, settings_path, "resources")

res = ezkl.compile_circuit(model_path, compiled_model_path, settings_path)
assert res == True
```
+++ JS
For performance reasons, you can only compile ONNX models using Lilith, python and cli environments. Stay tuned for updates!
+++ 

#### Creating the circuit

+++ CLI
Now, we use `setup` to create a proving and verifying key for our circuit, using the SRS and our compiled `.ezkl ` onnx model. 

```bash
ezkl setup -M network.ezkl --srs-path="/Users/user/.ezkl/srs/kzg17.srs" --vk-path=vk.key --pk-path=pk.key
```
This creates the verification key, proving key, and circuit settings in the locations you specify. 

> Note: The when using `ezkl get-srs`, the file is stored in `$HOME/.ezkl/srs`. You will need to locate it, and change the path/name as appropriate in the command above.

> Note: You can view the options associated to a subcommand such as `setup` by typing `ezkl setup` with no parameters. If you provide some but not all required parameters, `ezkl` will tell you what else it needs.
+++ Python
From the compiled model and SRS (structured reference string), we will setup the circuit parameters consisting of the proving and verifying keys. Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet.

```python
compiled_model_path = os.path.join('network.compiled')
srs_path = os.path.join('kzg.srs')
pk_path = os.path.join('test.pk')
vk_path = os.path.join('test.vk')

# get public srs from kzg ceremony, saved to srs path. 
res = ezkl.get_srs(srs_path, settings_path)

# setup the circuit and make sure the keys are generated afterwards. 
res = ezkl.setup(
        compiled_model_path,
        vk_path,
        pk_path,
        srs_path,
    )

assert res == True
assert os.path.isfile(vk_path)
assert os.path.isfile(pk_path)
```
+++ JS

The EZKL Engine npm package supports the setup command. Though we do not recommend it. We recommend using Lilith, python or cli environments for performance reasons. Stay tuned for updates!

===
+++


#### Making a proof
+++ CLI
First we generate a witness file.

```bash
ezkl gen-witness --compiled-circuit network.ezkl
```

Next we will generate a proof that the model was correctly run on private inputs (this is the default setting)..

```bash
ezkl prove --compiled-circuit network.ezkl
```
+++ Python
To generate a proof, we first need to make a witness file. We can do this by running a forward pass using the input data on the compiled model. 

We can use this witness, along with the compiled model, proving key and SRS to generate a proof that the model was correctly run on public inputs.

Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet.

```python
# generate witness
res = ezkl.gen_witness()
# generate proof
res = ezkl.prove()
```
+++ JS

Generate a witness file using the form rendered below. 

[!embed el="embed" aspect="1:1" width="400" height="465"](https://ezkljs-engine.vercel.app/gen-witness)
==- View Source Code
```typescript gen-witness.tsx
'use client'
import {
    FileInput,
    Label,
    Button,
    Alert,
    Spinner as _Spinner,
    Modal,
} from 'flowbite-react'
import React, { useEffect, useState } from 'react'
import { formDataSchema } from './parsers'
import { stringify } from 'json-bigint'
import { useSharedResources } from '../EngineContext'

export default function GenWitness() {
    const { engine, utils } = useSharedResources()
    const [openModal, setOpenModal] = useState<string | undefined>()
    const [alert, setAlert] = useState<string>('')
    const [warning, setWarning] = useState<string>('')
    const [loading, setLoading] = useState(false)
    const [witness, setWitness] = useState({})
    const [witnessResult, setWitnessResult] = useState<string>('')
    const [buffer, setBuffer] = useState<Uint8Array | null>(null)

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const formData = new FormData(e.currentTarget)

        const formInputs = {
            compiled_onnx: formData.get('compiled_onnx'),
            input: formData.get('input'),
        }
        // Validate form has valid inputs (zod)
        const validatedFormInputs = formDataSchema.safeParse(formInputs)

        if (warning) setWarning('')

        if (!validatedFormInputs.success) {
            setAlert('Please upload all files')
            return
        }

        // Clear alert and warning
        if (alert) setAlert('')

        // Missing data
        if (
            validatedFormInputs.data.compiled_onnx === null ||
            validatedFormInputs.data.input === null
        ) {
            setAlert('Please upload all files')
            return
        }

        let files = {
            compiled_onnx: validatedFormInputs.data.compiled_onnx,
            input: validatedFormInputs.data.input,
        }

        setLoading(true)

        /* ================== ENGINE API ====================== */
        utils
            .handleGenWitnessButton(files as { [key: string]: File })
            .then(({ output, executionTime }) => {
                setBuffer(output)

                // Update result based on the outcome
                setWitnessResult(
                    output
                        ? `Witness generation successful. Execution time: ${executionTime} ms`
                        : 'Witness generation failed',
                )
                const witness = engine.deserialize(output)
                console.log('witness', witness)
                setWitness(witness)
            })
            .catch((error) => {
                console.error('An error occurred:', error)
                setWarning(`Witness generation failed: ${error}`)
            })

        setLoading(false)
    }

    return (
        <div className='flex flex-column justify-around'>
            {buffer && !warning ? (
                <div className='flex flex-col justify-around'>
                    <h1 className='text-2xl mb-6 '>{witnessResult}</h1>

                    <div className='flex flex-col flex-grow w-full items-center justify-around'>
                        <Button
                            className='w-full flex-grow'
                            type='submit'
                            onClick={() => utils.handleFileDownload('witness.json', buffer)}
                        >
                            Download Witness
                        </Button>
                        <Button
                            className='w-full flex-grow mt-4'
                            onClick={() => setOpenModal('default')}
                            data-modal-target='witness-modal'
                            data-modal-toggle='witness-modal'
                        >
                            Show Witness
                        </Button>
                        <Button className='w-full flex-grow mt-4' onClick={() => setBuffer(null)}>
                            Reset
                        </Button>
                        <Modal
                            show={openModal === 'default'}
                            onClose={() => setOpenModal(undefined)}
                        >
                            <Modal.Header>Witness File Content: </Modal.Header>
                            <Modal.Body className='bg-black'>
                                <div className='mt-4 p-4 bg-black-100 rounded'>
                                    <pre className='blackspace-pre-wrap' style={{ fontSize: '13px' }}>
                                        {stringify(witness, null, 6)}
                                    </pre>
                                </div>
                            </Modal.Body>
                        </Modal>
                    </div>
                </div>
            ) : loading ? (
                <Spinner />
            ) : (
                <div className='flex flex-col w-full items-center space-y-4'>
                    <WitnessArtifactForm
                        handleSubmit={handleSubmit}
                        alert={alert}
                        warning={warning}
                    />
                    <Button
                        type='submit'
                        color='dark'
                        className='self-center mt-4 w-full'
                        onClick={() => populateWithSampleFiles()}
                    >
                        Populate with sample files
                    </Button>
                </div>
            )}
        </div>
    )
}
// UI Component
function Spinner() {
    return (
        <div className='h-full flex items-center'>
            <_Spinner size='3xl' className='w-28 lg:w-44' />
        </div>
    )
}

async function populateWithSampleFiles() {
    // Helper to assert that the element is not null
    function assertElement<T extends Element>(
        element: T | null,
    ): asserts element is T {
        if (element === null) {
            throw new Error('Element not found')
        }
    }

    // Names of the sample files in the public directory
    const sampleFileNames: { [key: string]: string } = {
        compiled_onnx: 'test_network.compiled',
        input: 'input.json',
    }

    // Helper function to fetch and create a file object from a public URL
    const fetchAndCreateFile = async (
        path: string,
        filename: string,
    ): Promise<File> => {
        const response = await fetch(path)
        const blob = await response.blob()
        return new File([blob], filename, { type: blob.type })
    }

    // Fetch each sample file and create a File object
    const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
        fetchAndCreateFile(`/data/1l_mlp/${filename}`, filename),
    )

    // Wait for all files to be fetched and created
    const files = await Promise.all(filePromises)

    // Select the file input elements and assign the FileList to each
    const compiledOnnxInput =
        document.querySelector<HTMLInputElement>('#compiled_onnx')
    const input = document.querySelector<HTMLInputElement>('#input')

    // Assert that the elements are not null
    assertElement(compiledOnnxInput)
    assertElement(input)

    // Create a new DataTransfer to hold the files
    let dataTransfers: DataTransfer[] = []
    files.forEach((file, idx) => {
        const dataTransfer = new DataTransfer()
        dataTransfer.items.add(file)
        dataTransfers[idx] = dataTransfer
    })

    compiledOnnxInput.files = dataTransfers[0].files
    input.files = dataTransfers[1].files
}

function WitnessArtifactForm({
    handleSubmit,
    alert,
    warning,
}: {
    handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
    alert: string
    warning: string
}) {
    return (
        <div className='flex flex-col'>
            <h1 className='text-2xl mb-6 '>Witnessing</h1>
            {alert && (
                <Alert color='info' className='mb-6'>
                    {alert}
                </Alert>
            )}
            {warning && (
                <Alert color='warning' className='mb-6'>
                    {warning}
                </Alert>
            )}
            <form
                onSubmit={handleSubmit}
                className='flex flex-col flex-grow justify-between'
            >
                {/* COMPILED ONNX */}
                <div>
                    <Label
                        color='white'
                        htmlFor='compiled_onnx'
                        value='Select Compiled Onnx File'
                    />
                    <FileInput id='compiled_onnx' name='compiled_onnx' className='my-4' />
                </div>
                {/* INPUT */}
                <div>
                    <Label color='white' htmlFor='input' value='Select Input File' />
                    <FileInput id='input' name='input' className='my-4' />
                </div>
                <Button type='submit' color='dark' className='w-full self-center mt-4'>
                    Generate Witness
                </Button>
            </form>
        </div>
    )
}

```
===
Use the form rendered below to generate ZKML proofs in the browser right now :)

[!embed el="embed" aspect="1:1" width="400" height="660"](https://ezkljs-engine.vercel.app/prove)
==- View Source Code
```typescript prove.tsx
// Example for pages/Page1.js
'use client'
import {
  FileInput,
  Label,
  Button,
  Alert,
  Spinner as _Spinner,
  Modal,
} from 'flowbite-react'
import React, { useState } from 'react'
import { formDataSchemaProve } from './parsers'
import { stringify } from 'json-bigint'
import { useSharedResources } from '../EngineContext'

export default function Prove() {
  const { engine, utils } = useSharedResources()
  const [openModal, setOpenModal] = useState<string | undefined>()
  const props = { openModal, setOpenModal }
  const [alertProof, setAlertProof] = useState<string>('')
  const [warningProof, setWarningProof] = useState<string>('')
  const [loading, setLoading] = useState(false)
  const [proofResult, setProofResult] = useState('')
  const [proof, setProof] = useState({})
  const [buffer, setBuffer] = useState<Uint8Array | null>(null)

  const handleSubmitProve = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)

    const formInputs = {
      witness: formData.get('witness'),
      pk: formData.get('pk'),
      compiled_onnx: formData.get('compiled_onnx'),
      srs: formData.get('srs'),
    }
    // Validate form has valid inputs (zod)
    const validatedFormInputs = formDataSchemaProve.safeParse(formInputs)

    if (warningProof) setWarningProof('')

    if (!validatedFormInputs.success) {
      setAlertProof('Please upload all files')
      return
    }

    // Clear alert and warning
    if (alertProof) setAlertProof('')

    // Missing data
    if (
      validatedFormInputs.data.witness === null ||
      validatedFormInputs.data.pk === null ||
      validatedFormInputs.data.compiled_onnx === null ||
      validatedFormInputs.data.srs === null
    ) {
      setAlertProof('Please upload all files')
      return
    }

    setLoading(true)

    // create file object
    const files = {
      data: validatedFormInputs.data.witness,
      pk: validatedFormInputs.data.pk,
      model: validatedFormInputs.data.compiled_onnx,
      srs: validatedFormInputs.data.srs,
    }
    /* ================== ENGINE API ====================== */
    utils
      .handleGenProofButton(files as { [key: string]: File })
      .then(({ output, executionTime }) => {
        setBuffer(output)

        // Update result based on the outcome
        setProofResult(
          output
            ? `Proof generation successful. Execution time: ${executionTime} ms`
            : 'Proof generation failed',
        )
        // Deseralize proof buffer
        // TODO - uncomment this line once a new engine bundle is relased
        // with patch to web based serialize/deserialize methods.
        const proof = engine.deserialize(output)
        console.log('proof', proof)
        setProof(proof)
      })
      .catch((error) => {
        console.error('An error occurred:', error)
        setWarningProof(`Proof generation failed: ${error}`)
      })

    setLoading(false)
  }

  return (
    <div className='flex flex-column justify-around'>
      {buffer && !warningProof ? (
        <div className='flex flex-col justify-around'>
          <h1 className='text-2xl mb-6 '>{proofResult}</h1>
          <div className='flex flex-col flex-grow w-full items-center justify-around'>
            <Button
              className='w-full flex-grow'
              type='submit'
              onClick={() => utils.handleFileDownload('test.pf', buffer)}
            >
              Download Proof File
            </Button>
            <Button
              className='w-full flex-grow mt-4'
              onClick={() => props.setOpenModal('default')}
              data-modal-target='witness-modal'
              data-modal-toggle='witness-modal'
            >
              Show Proof
            </Button>
            <Button className='w-full flex-grow mt-4' onClick={() => setBuffer(null)}>
              Reset
            </Button>
            <Modal
              show={props.openModal === 'default'}
              onClose={() => props.setOpenModal(undefined)}
            >
              <Modal.Header>Proof File Content: </Modal.Header>
              <Modal.Body className='bg-black'>
                <div className='mt-4 p-4 bg-black-100 rounded'>
                  <pre className='blackspace-pre-wrap' style={{ fontSize: '13px' }}>
                    {stringify(proof, null, 6)}
                  </pre>
                </div>
              </Modal.Body>
            </Modal>
          </div>
        </div>
      ) : loading ? (
        <Spinner />
      ) : (
        <div className='flex flex-col w-full items-center space-y-4'>
          <div className='flex w-full items-stretch space-x-8'>
            <ProvingArtifactForm
              handleSubmit={handleSubmitProve}
              alert={alertProof}
              warning={warningProof}
            />
          </div>
          <Button
            type='submit'
            color='dark'
            className='self-center mt-4 w-full'
            onClick={() => populateWithSampleFiles()}
          >
            Populate with sample files
          </Button>
        </div>
      )}
    </div>
  )
}
// UI Component
function Spinner() {
  return (
    <div className='h-full flex items-center'>
      <_Spinner size='3xl' className='w-28 lg:w-44' />
    </div>
  )
}

async function populateWithSampleFiles() {
  // Helper to assert that the element is not null
  function assertElement<T extends Element>(
    element: T | null,
  ): asserts element is T {
    if (element === null) {
      throw new Error('Element not found')
    }
  }

  // Names of the sample files in the public directory
  const sampleFileNames: { [key: string]: string } = {
    witness: 'test.witness.json',
    pk: 'test.provekey',
    compiled_onnx: 'test_network.compiled',
    srs: 'kzg',
  }

  // Helper function to fetch and create a file object from a public URL
  const fetchAndCreateFile = async (
    path: string,
    filename: string,
  ): Promise<File> => {
    const response = await fetch(path)
    const blob = await response.blob()
    return new File([blob], filename, { type: blob.type })
  }

  // Fetch each sample file and create a File object
  const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
    fetchAndCreateFile(`/data/1l_mlp/${filename}`, filename),
  )

  // Wait for all files to be fetched and created
  const files = await Promise.all(filePromises)

  // Select the file input elements and assign the FileList to each
  const witness = document.querySelector<HTMLInputElement>('#witness')
  const pk = document.querySelector<HTMLInputElement>('#pk')
  const compiled_onnx =
    document.querySelector<HTMLInputElement>('#compiled_onnx')
  const srsProve = document.querySelector<HTMLInputElement>('#srs_prove')

  // Assert that the elements are not null
  assertElement(witness)
  assertElement(pk)
  assertElement(compiled_onnx)
  assertElement(srsProve)

  // Create a new DataTransfer to hold the files
  let dataTransfers: DataTransfer[] = []
  files.forEach((file, idx) => {
    const dataTransfer = new DataTransfer()
    dataTransfer.items.add(file)
    dataTransfers[idx] = dataTransfer
  })

  witness.files = dataTransfers[0].files
  pk.files = dataTransfers[1].files
  compiled_onnx.files = dataTransfers[2].files
  srsProve.files = dataTransfers[3].files
}

function ProvingArtifactForm({
  handleSubmit,
  alert,
  warning,
}: {
  handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
  alert: string
  warning: string
}) {
  return (
    <div className='flex flex-col'>
      <h1 className='text-2xl mb-6 '>Proving</h1>
      {alert && (
        <Alert color='info' className='mb-6'>
          {alert}
        </Alert>
      )}
      {warning && (
        <Alert color='warning' className='mb-6'>
          {warning}
        </Alert>
      )}
      <form
        onSubmit={handleSubmit}
        className='flex flex-col flex-grow  justify-between'
      >
        {/* WITNESS */}
        <div>
          <Label color='white' htmlFor='witness' value='Select Witness File' />
          <FileInput id='witness' name='witness' className='my-4' />
        </div>
        {/* PK */}
        <div>
          <Label color='white' htmlFor='pk' value='Select Proving Key File' />
          <FileInput id='pk' name='pk' className='my-4' />
        </div>
        {/* COMPILED ONNX MODEL */}
        <div>
          <Label
            color='white'
            htmlFor='compiled_onnx'
            value='Select Compiled Onnx File'
          />
          <FileInput id='compiled_onnx' name='compiled_onnx' className='my-4' />
        </div>
        {/* SRS */}
        <div>
          <Label color='white' htmlFor='srs' value='Select SRS File' />
          <FileInput id='srs_prove' name='srs' className='my-4' />
        </div>
        <Button type='submit' color='dark' className='w-full self-center mt-4'>
          Generate Proof
        </Button>
      </form>
    </div>
  )
}


```
===
+++ 

#### Verification
+++ CLI
We can then verify our generated proof with the `verify` command:
```bash
ezkl verify
```
+++ Python
Using the proof, settings, verification key and SRS, we can verify our proof.

Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet.

```python
res = ezkl.verify()
assert res == True
```
+++ JS
Use the form rendered below to verify ZKML proofs in the browser right now :)

[!embed el="embed" aspect="1:1" width="400" height="660"](https://ezkljs-engine.vercel.app/verify)

==- View Source Code
```typescript verify.tsx
'use client'
import {
  FileInput,
  Label,
  Button,
  Alert,
  Spinner as _Spinner,
} from 'flowbite-react'
import React, { useState } from 'react'
import { formDataSchemaVerify } from './parsers'
import { useSharedResources } from '../EngineContext'

export default function Verify() {
  const { utils } = useSharedResources()
  const [alertVerify, setAlertVerify] = useState<string>('')
  const [warningVerify, setWarningVerify] = useState<string>('')
  const [loading, setLoading] = useState(false)
  const [verifyResult, setVerifyResult] = useState<string>('')

  const handleSubmitVerify = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)

    const formInputs = {
      proof: formData.get('proof'),
      vk: formData.get('vk'),
      settings: formData.get('settings'),
      srs: formData.get('srs'),
    }
    // Validate form has valid inputs (zod)
    const validatedFormInputs = formDataSchemaVerify.safeParse(formInputs)

    if (warningVerify) setWarningVerify('')

    if (!validatedFormInputs.success) {
      setAlertVerify('Please upload all files')
      return
    }

    // Clear alert and warning
    if (alertVerify) setAlertVerify('')

    // Missing data
    if (
      validatedFormInputs.data.proof === null ||
      validatedFormInputs.data.vk === null ||
      validatedFormInputs.data.settings === null ||
      validatedFormInputs.data.srs === null
    ) {
      setAlertVerify('Please upload all files')
      return
    }

    setLoading(true)

    // create file object
    const files = {
      proof: validatedFormInputs.data.proof,
      vk: validatedFormInputs.data.vk,
      settings: validatedFormInputs.data.settings,
      srs: validatedFormInputs.data.srs,
    }
    /* ================== ENGINE API ====================== */
    utils
      .handleVerifyButton(files as { [key: string]: File })
      .then(({ output, executionTime }) => {
        // Update result based on the outcome
        setVerifyResult(
          output
            ? 'Verification successful. Execution time: ' +
            executionTime +
            ' ms'
            : 'Verification failed',
        )
      })
      .catch((error) => {
        console.error('An error occurred:', error)
        setWarningVerify(`Verification process failed with an error: ${error}`)
      })

    setLoading(false)
  }

  return (
    <div className='flex flex-column justify-around'>
      {verifyResult && !warningVerify ? (
        <div className='flex flex-col justify-around'>
          <h1 className='text-2xl mb-4 '>{verifyResult}</h1>
          <div className='flex flex-col flex-grow w-full items-center justify-around'>
            <Button className='w-full flex-grow' onClick={() => setVerifyResult('')}>
              Reset
            </Button>
          </div>
        </div>
      ) : loading ? (
        <Spinner />
      ) : (
        <div className='flex flex-col w-full items-center space-y-4'>
          <div className='flex w-full items-stretch space-x-8'>
            <VerifyingArtifactForm
              handleSubmit={handleSubmitVerify}
              alert={alertVerify}
              warning={warningVerify}
            />
          </div>
          <Button
            type='submit'
            color='dark'
            className='self-center mt-4 w-full'
            onClick={() => populateWithSampleFiles()}
          >
            Populate with sample files
          </Button>
        </div>
      )}
    </div>
  )
}
// UI Component
function Spinner() {
  return (
    <div className='h-full flex items-center'>
      <_Spinner size='3xl' className='w-28 lg:w-44' />
    </div>
  )
}

async function populateWithSampleFiles() {
  // Helper to assert that the element is not null
  function assertElement<T extends Element>(
    element: T | null,
  ): asserts element is T {
    if (element === null) {
      throw new Error('Element not found')
    }
  }

  // Names of the sample files in the public directory
  const sampleFileNames: { [key: string]: string } = {
    srs: 'kzg',
    proof: 'test.pf',
    settings: 'settings.json',
    vk: 'test.key',
  }

  // Helper function to fetch and create a file object from a public URL
  const fetchAndCreateFile = async (
    path: string,
    filename: string,
  ): Promise<File> => {
    const response = await fetch(path)
    const blob = await response.blob()
    return new File([blob], filename, { type: blob.type })
  }

  // Fetch each sample file and create a File object
  const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
    fetchAndCreateFile(`/data/1l_mlp/${filename}`, filename),
  )

  // Wait for all files to be fetched and created
  const files = await Promise.all(filePromises)

  // Select the file input elements and assign the FileList to each
  const proof = document.querySelector<HTMLInputElement>('#proof')
  const settings = document.querySelector<HTMLInputElement>('#settings')
  const vk = document.querySelector<HTMLInputElement>('#vk')
  const srsVerify = document.querySelector<HTMLInputElement>('#srs_verify')

  // Assert that the elements are not null
  assertElement(proof)
  assertElement(settings)
  assertElement(vk)
  assertElement(srsVerify)

  // Create a new DataTransfer to hold the files
  let dataTransfers: DataTransfer[] = []
  files.forEach((file, idx) => {
    const dataTransfer = new DataTransfer()
    dataTransfer.items.add(file)
    dataTransfers[idx] = dataTransfer
  })

  srsVerify.files = dataTransfers[0].files
  proof.files = dataTransfers[1].files
  settings.files = dataTransfers[2].files
  vk.files = dataTransfers[3].files
}

function VerifyingArtifactForm({
  handleSubmit,
  alert,
  warning,
}: {
  handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
  alert: string
  warning: string
}) {
  return (
    <div className='flex flex-col'>
      <h1 className='text-2xl mb-6 '>Verifying</h1>
      {alert && (
        <Alert color='info' className='mb-6'>
          {alert}
        </Alert>
      )}
      {warning && (
        <Alert color='warning' className='mb-6'>
          {warning}
        </Alert>
      )}
      <form
        onSubmit={handleSubmit}
        className='flex flex-col flex-grow justify-between'
      >
        {/* PROOF */}
        <div>
          <Label color='white' htmlFor='proof' value='Select Proof File' />
          <FileInput id='proof' name='proof' className='my-4' />
        </div>
        {/* SETTINGS */}
        <div>
          <Label
            color='white'
            htmlFor='settings'
            value='Select Settings File'
          />
          <FileInput id='settings' name='settings' className='my-4' />
        </div>
        {/* VK */}
        <div>
          <Label color='white' htmlFor='vk' value='Select VK File' />
          <FileInput id='vk' name='vk' className='my-4' />
        </div>
        {/* SRS */}
        <div>
          <Label color='white' htmlFor='srs' value='Select SRS File' />
          <FileInput id='srs_verify' name='srs' className='my-4' />
        </div>
        <Button type='submit' color='dark' className='w-full self-center mt-4'>
          Verify
        </Button>
      </form>
    </div>
  )
}

```
===
+++ In-browser EVM verifying
Use the form rendered below to verify ZKML proofs in the browser right now :)

[!embed el="embed" aspect="1:1" width="400" height="565"](https://ezkljs-engine.vercel.app/in-browser-evm-verify)

==- View Source Code
```typescript inBrowserEvmVerify.tsx
'use client'
import {
  FileInput,
  Label,
  Button,
  Alert,
  Select,
  Spinner as _Spinner,
} from 'flowbite-react'
import React, { useState } from 'react'
import { formDataSchemaEvmVerify } from './parsers'
import { useSharedResources } from '../EngineContext'

enum Hardfork {
  Chainstart = 'chainstart',
  Homestead = 'homestead',
  Dao = 'dao',
  TangerineWhistle = 'tangerineWhistle',
  SpuriousDragon = 'spuriousDragon',
  Byzantium = 'byzantium',
  Constantinople = 'constantinople',
  Petersburg = 'petersburg',
  Istanbul = 'istanbul',
  MuirGlacier = 'muirGlacier',
  Berlin = 'berlin',
  London = 'london',
  ArrowGlacier = 'arrowGlacier',
  GrayGlacier = 'grayGlacier',
  MergeForkIdTransition = 'mergeForkIdTransition',
  Paris = 'paris',
  Shanghai = 'shanghai',
  Cancun = 'cancun',
}

export default function InBrowserEvmVerify() {
  const { utils } = useSharedResources()
  const [alertVerify, setAlertVerify] = useState<string>('')
  const [warningVerify, setWarningVerify] = useState<string>('')
  const [loading, setLoading] = useState(false)
  const [evmVerifyResult, setEvmVerifyResult] = useState<string>('')

  const handleSubmitVerify = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)

    const formInputs = {
      proof: formData.get('proof'),
      bytecode_verifier: formData.get('bytecode_verifier'),
      evm_version: formData.get('evm_version'),
    }
    // Validate form has valid inputs (zod)
    const validatedFormInputs = formDataSchemaEvmVerify.safeParse(formInputs)

    if (warningVerify) setWarningVerify('')

    if (!validatedFormInputs.success) {
      setAlertVerify('Please upload all files')
      return
    }

    // Clear alert and warning
    if (alertVerify) setAlertVerify('')
    if (warningVerify) setWarningVerify('')

    // Missing data
    if (
      validatedFormInputs.data.proof === null ||
      validatedFormInputs.data.bytecode_verifier === null ||
      validatedFormInputs.data.evm_version === '' ||
      validatedFormInputs.data.evm_version === null
    ) {
      setAlertVerify('Please upload all files')
      return
    }

    setLoading(true)

    // create file object
    const files = {
      proof: validatedFormInputs.data.proof,
      bytecodeVerifier: validatedFormInputs.data.bytecode_verifier,
    }
    /* ================== ENGINE API ====================== */
    utils
      .handleEvmVerifyButton(
        files as { [key: string]: File },
        validatedFormInputs.data.evm_version as Hardfork,
      )
      .then(({ output, executionTime }) => {
        // Update result based on the outcome
        setEvmVerifyResult(
          output
            ? 'In-browser EVM verification successful. Execution time: ' +
            executionTime +
            ' ms'
            : 'In-browser EVM verification failed',
        )
      })
      .catch((error) => {
        console.error('An error occurred:', error)
        setEvmVerifyResult('An error occurred: ' + error)
      })

    setLoading(false)
  }

  return (
    <div className='flex flex-column justify-around'>
      {evmVerifyResult && !warningVerify ? (
        <div className='flex flex-col justify-around'>
          <h1 className='text-2xl mb-4 '>{evmVerifyResult}</h1>
          <div className='flex flex-col flex-grow w-full items-center justify-around'>
            <Button className='w-full flex-grow' onClick={() => setEvmVerifyResult('')}>
              Reset
            </Button>
          </div>
        </div>
      ) : loading ? (
        <Spinner />
      ) : (
        <div className='flex flex-col w-full items-center space-y-4'>
          <VerifyingArtifactForm
            handleSubmit={handleSubmitVerify}
            alert={alertVerify}
            warning={warningVerify}
          />
          <Button
            type='submit'
            color='dark'
            className='self-center mt-4 w-full'
            onClick={() => populateWithSampleFiles()}
          >
            Populate with sample files
          </Button>
        </div>
      )}
    </div>
  )
}
// UI Component
function Spinner() {
  return (
    <div className='h-full flex items-center'>
      <_Spinner size='3xl' className='w-28 lg:w-44' />
    </div>
  )
}

async function populateWithSampleFiles() {
  // Helper to assert that the element is not null
  function assertElement<T extends Element>(
    element: T | null,
  ): asserts element is T {
    if (element === null) {
      throw new Error('Element not found')
    }
  }

  // Names of the sample files in the public directory
  const sampleFileNames: { [key: string]: string } = {
    proof: 'evm_verify.pf',
    bytecode: 'bytecode.code',
  }

  // Helper function to fetch and create a file object from a public URL
  const fetchAndCreateFile = async (
    path: string,
    filename: string,
  ): Promise<File> => {
    const response = await fetch(path)
    const blob = await response.blob()
    return new File([blob], filename, { type: blob.type })
  }

  // Fetch each sample file and create a File object
  const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
    fetchAndCreateFile(`/data/1l_mlp/${filename}`, filename),
  )

  // Wait for all files to be fetched and created
  const files = await Promise.all(filePromises)

  // Select the file input elements and assign the FileList to each
  const proof = document.querySelector<HTMLInputElement>('#proof')
  const bytecode =
    document.querySelector<HTMLInputElement>('#bytecode_verifier')

  // Assert that the elements are not null
  assertElement(proof)
  assertElement(bytecode)

  // Create a new DataTransfer to hold the files
  let dataTransfers: DataTransfer[] = []
  files.forEach((file, idx) => {
    const dataTransfer = new DataTransfer()
    dataTransfer.items.add(file)
    dataTransfers[idx] = dataTransfer
  })

  proof.files = dataTransfers[0].files
  bytecode.files = dataTransfers[1].files
}

function VerifyingArtifactForm({
  handleSubmit,
  alert,
  warning,
}: {
  handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
  alert: string
  warning: string
}) {
  const [selectedVersion, setSelectedVersion] = useState<Hardfork>(
    Hardfork.Istanbul,
  )

  const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selectedValue = e.target.value as Hardfork
    setSelectedVersion(selectedValue)
  }
  return (
    <div className='flex flex-col'>
      <h1 className='text-2xl mb-6 '>In-Browser Evm Verifying</h1>
      {alert && (
        <Alert color='info' className='mb-6'>
          {alert}
        </Alert>
      )}
      {warning && (
        <Alert color='warning' className='mb-6'>
          {warning}
        </Alert>
      )}
      <form
        onSubmit={handleSubmit}
        className='flex flex-col flex-grow  justify-between'
      >
        {/* PROOF */}
        <div>
          <Label color='white' htmlFor='proof' value='Select Proof File' />
          <FileInput id='proof' name='proof' className='my-4' />
        </div>
        {/* VERIFIER BYTECODE */}
        <div>
          <Label
            color='white'
            htmlFor='bytecode_verifier'
            value='Select Evm Verifier Bytecode'
          />
          <FileInput
            id='bytecode_verifier'
            name='bytecode_verifier'
            className='my-4'
          />
        </div>
        {/* EVM VERSION */}
        <div>
          <Label
            color='white'
            htmlFor='evm_version'
            value='Select Evm Version'
          />
          <Select
            id='evm_version'
            name='evm_version'
            className='my-4'
            onChange={handleChange}
            value={selectedVersion}
          >
            {Object.keys(Hardfork).map((key) => (
              <option key={key} value={Hardfork[key as keyof typeof Hardfork]}>
                {key}
              </option>
            ))}
          </Select>
        </div>
        <Button type='submit' color='dark' className='w-full self-center mt-4'>
          Verify
        </Button>
      </form>
    </div>
  )
}
```
===
+++



