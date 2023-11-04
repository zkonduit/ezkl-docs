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
To display `ezkl`'s understanding of the model in the CLI, run:

```bash
ezkl table -M network.onnx
```

#### Setting circuit parameters

+++ CLI
Our circuit is configured with the `settings.json` file. This is created with the `gen-settings` command. 
```bash
ezkl gen-settings -M network.onnx
```
This will produce a `settings.json` file you can use for your circuit. However, you can fine-tune your circuit to optimize for accuracy or CPU/memory usage with the `calibrate-settings` command:
```bash
ezkl calibrate-settings -M network.onnx -D input.json --target resources
```
In this example, we set the `--target` to **"resources"** so that we can optimize for CPU and memory usage. The other option is **"accuracy"**, which optimizes for accuracy given the fixed point representation of the input model. Our circuit parameters are generated, then saved to `settings.json`. You can pass a `--settings-path` to read from an existing settings file, and only modify the parts changed by calibration (e.g. leaving visibility or tolerance unchanged). You can customize this file and even change the way it is generated. Learn more about `gen-settings` and `calibrate-settings` in the [Commands](https://docs.ezkl.xyz/about_ezkl/commands/) section.

Download the appropriate SRS:
```bash
ezkl get-srs -S settings.json
```
+++ Python
From the `network.onnx` onnx file, we will create a `settings.json` file that uses the `py_run_args` file to specify the visibility of the inputs, outputs and paramaters of the model. 

Once we have created the settings file, we can calibrate it using the `ezkl.calibrate_settings` command to optimize for either `accuracy` or `resources` using the input to the model (`input.json`), and the model itself (`network.onnx`). The accurary target will optimize the circuit for accuracy given the fixed point representation of the input model. The resources target will optimize the circuit for CPU and memory usage.

Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet. 

```python
model_path = os.path.join('network.onnx')
settings_path = os.path.join('settings.json')
data_path = os.path.join('input.json')

py_run_args = ezkl.PyRunArgs()
py_run_args.input_visibility = "public"
py_run_args.output_visibility = "public"
py_run_args.param_visibility = "fixed" # "fixed" for params means that the committed to params are used for all proofs

res = ezkl.gen_settings(model_path, settings_path, py_run_args=py_run_args)
assert res == True

res = await ezkl.calibrate_settings(data_path, model_path, settings_path, "resources")
assert res == True
```
+++ JS
For performance reaons, you can only generate settings using the hub, python and cli environments. Stay tuned for updates!
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
For performance reaons, you can only compile ONNX models using the hub, python and cli environments. Stay tuned for updates!
+++ 

#### Creating the circuit

+++ CLI
Now, we use `setup` to create a proving and verifying key for our circuit, using the SRS and our compiled `.ezkl ` onnx model. 

```bash
ezkl setup -M network.ezkl --srs-path=kzg.srs --vk-path=vk.key --pk-path=pk.key
```
This creates the verification key, proving key, and circuit settings in the locations you specify. 

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

The EZKL Engine npm package supports the setup command. 

Use the form below to generate a verifying key and proving key for your circuit in a browser right now :)

> Note: We designed the API of these methods such that one needs to create a verifying key before they can create a proving key, as we imagine its more useful in most application to create a verifying key instead of just a proving key in a browser context.

[!embed el="embed" aspect="1:1" width="740" height="560"](https://ezkljs-engine.vercel.app/setup)
==- View Source Code
```typescript setup.tsx
'use client'
import {
    FileInput,
    Label,
    Button,
    Alert,
    Spinner as _Spinner
} from 'flowbite-react'
import React, { useState } from 'react'
import { formDataSchemaGenVk, formDataSchemaGenPk } from './parsers'
import { useSharedResources } from '../EngineContext';

export default function Setup() {
    const { engine, utils } = useSharedResources();
    const [openModal, setOpenModal] = useState<string | undefined>();
    const props = { openModal, setOpenModal };
    const [alertGenVk, setAlertGenVk] = useState<string>('')
    const [warningGenVk, setWarningGenVk] = useState<string>('')
    const [alertGenPk, setAlertGenPk] = useState<string>('')
    const [warningGenPk, setWarningGenPk] = useState<string>('')
    const [loading, setLoading] = useState(false)
    const [genVkResult, setGenVkResult] = useState('')
    const [genPkResult, setGenPkResult] = useState('')
    const [bufferVk, setBufferVk] = useState<Uint8Array | null>(null)
    const [bufferPk, setBufferPk] = useState<Uint8Array | null>(null)

    const handleSubmitGenVk = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const formData = new FormData(e.currentTarget)

        const formInputs = {
            compiled_onnx: formData.get('compiled_onnx'),
            srs: formData.get('srs'),
        }
        // Validate form has valid inputs (zod)
        const validatedFormInputs = formDataSchemaGenVk.safeParse(formInputs)

        if (warningGenVk) setWarningGenVk('')

        if (!validatedFormInputs.success) {
            setAlertGenVk('Please upload all files')
            return
        }

        // Clear alert and warning
        if (alertGenVk) setAlertGenVk('')

        // Missing data
        if (
            validatedFormInputs.data.compiled_onnx === null ||
            validatedFormInputs.data.srs === null
        ) {
            setAlertGenVk('Please upload all files')
            return
        }

        setLoading(true)

        // create file object
        const files = {
            compiled_onnx: validatedFormInputs.data.compiled_onnx,
            srs: validatedFormInputs.data.srs,
        }
        /* ================== ENGINE API ====================== */
        utils.handleGenVkButton(files as { [key: string]: File })
            .then(({ output, executionTime }) => {
                setBufferVk(output)



                // Update result based on the outcome
                setGenVkResult(
                    output
                        ? `Vk generation successful. Execution time: ${executionTime} ms`
                        : "Vk generation failed"
                )
            })
            .catch((error) => {
                console.error('An error occurred:', error)
                setWarningGenVk(`Vk generation failed: ${error}`)
            })

        setLoading(false)
    }
    const handleSubmitGenPk = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const formData = new FormData(e.currentTarget)

        const formInputs = {
            vk: formData.get('vk'),
            compiled_onnx: formData.get('compiled_onnx'),
            srs: formData.get('srs'),
        }
        // Validate form has valid inputs (zod)
        const validatedFormInputs = formDataSchemaGenPk.safeParse(formInputs)

        if (warningGenPk) setWarningGenPk('')


        if (!validatedFormInputs.success) {
            setAlertGenPk('Please upload all files')
            return
        }

        // Clear alert and warning
        if (alertGenPk) setAlertGenPk('')

        // Missing data
        if (
            validatedFormInputs.data.vk === null ||
            validatedFormInputs.data.compiled_onnx === null ||
            validatedFormInputs.data.srs === null
        ) {
            setAlertGenPk('Please upload all files')
            return
        }

        setLoading(true)
        console.log("hi")

        // create file object
        const files = {
            vk: validatedFormInputs.data.vk,
            compiled_onnx: validatedFormInputs.data.compiled_onnx,
            srs: validatedFormInputs.data.srs
        }
        /* ================== ENGINE API ====================== */
        utils.handleGenPkButton(files as { [key: string]: File })
            .then(({ output, executionTime }) => {
                setBufferPk(output)
                // Update result based on the outcome
                setGenPkResult(
                    output
                        ? 'Pk generation successful. Execution time: ' + executionTime + ' ms'
                        : 'Pk generation failed'
                )
            })
            .catch((error) => {
                console.error('An error occurred:', error)
                setWarningGenPk(`Pk process failed with an error: ${error}`)
            })

        setLoading(false)
    }


    return (
        <div className='flex flex-col justify-center items-center h-5/6 pb-20'>
            {bufferVk && !warningGenVk ? (
                <div className='w-10/12 flex flex-col'>
                    <h1 className='text-2xl mb-6 '>{genVkResult}</h1>
                    <div className="flex w-full justify-center pt-7">
                        <Button
                            className="w-1/2 mr-3"
                            type='submit'
                            onClick={() => utils.handleFileDownload('test.vk', bufferVk)}
                        >
                            Download Vk File
                        </Button>
                        <Button
                            className="w-1/2"
                            onClick={() => setBufferVk(null)}
                        >
                            Reset
                        </Button>
                    </div>
                </div>
            ) : bufferPk && !warningGenPk ? (
                <div className='w-10/12 flex flex-col'>
                    <h1 className='text-2xl mb-6 '>{genPkResult}</h1>
                    <div className="flex w-full justify-center pt-7">
                        <Button
                            className="w-1/2 mr-3"
                            type='submit'
                            onClick={() => utils.handleFileDownload('test.pk', bufferPk)}
                        >
                            Download Pk File
                        </Button>
                        <Button
                            className="w-1/2"
                            onClick={() => setBufferPk(null)}
                        >
                            Reset
                        </Button>
                    </div>
                </div>
            ) : loading ? (
                <Spinner />
            ) : (
                <div className='flex flex-col justify-between w-full items-center space-y-4'>
                    <div className='flex justify-between w-full items-stretch space-x-8'>
                        <GenVkArtifactForm handleSubmit={handleSubmitGenVk} alert={alertGenVk} warning={warningGenVk} />
                        <GenPkArtifactForm handleSubmit={handleSubmitGenPk} alert={alertGenPk} warning={warningGenPk} />
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
    );
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
    function assertElement<T extends Element>(element: T | null): asserts element is T {
        if (element === null) {
            throw new Error('Element not found');
        }
    }

    // Names of the sample files in the public directory
    const sampleFileNames: { [key: string]: string } = {
        compiled_onnx: 'test_network.compiled',
        srs: 'kzg',
        vk: 'test.key',
    };

    // Helper function to fetch and create a file object from a public URL
    const fetchAndCreateFile = async (path: string, filename: string): Promise<File> => {
        const response = await fetch(path);
        const blob = await response.blob();
        return new File([blob], filename, { type: blob.type });
    };

    // Fetch each sample file and create a File object
    const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
        fetchAndCreateFile(`/data/${filename}`, filename)
    );

    // Wait for all files to be fetched and created
    const files = await Promise.all(filePromises);

    // Select the file input elements and assign the FileList to each
    const compiledOnnxInputVk = document.querySelector<HTMLInputElement>('#compiled_onnx_vk');
    const srsInputVk = document.querySelector<HTMLInputElement>('#srs_vk');
    const compiledOnnxInputPk = document.querySelector<HTMLInputElement>('#compiled_onnx_pk');
    const srsInputPk = document.querySelector<HTMLInputElement>('#srs_pk');
    const vkInput = document.querySelector<HTMLInputElement>('#vk');

    // Assert that the elements are not null
    assertElement(compiledOnnxInputVk);
    assertElement(srsInputVk);
    assertElement(compiledOnnxInputPk);
    assertElement(srsInputPk);
    assertElement(vkInput);

    // Create a new DataTransfer to hold the files
    let dataTransfers: DataTransfer[] = [];
    files.forEach(
        (file, idx) => {
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(file)
            dataTransfers[idx] = dataTransfer;
        }

    );


    compiledOnnxInputVk.files = dataTransfers[0].files;
    srsInputVk.files = dataTransfers[1].files;
    compiledOnnxInputPk.files = dataTransfers[0].files;
    srsInputPk.files = dataTransfers[1].files;
    vkInput.files = dataTransfers[2].files;
}


function GenVkArtifactForm({
    handleSubmit,
    alert,
    warning
}: {
    handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
    alert: string
    warning: string
}) {
    return (
        <div className='flex flex-col'>
            <h1 className='text-2xl mb-6 '>Generate Verifying Key</h1>
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
                {/* COMPILED ONNX MODEL */}
                <div>
                    <Label color="white" htmlFor='compiled_onnx' value='Select Compiled Onnx File' />
                    <FileInput
                        id='compiled_onnx_vk'
                        name='compiled_onnx'
                        className='my-4'
                    />
                </div>
                {/* SRS */}
                <div>
                    <Label color="white" htmlFor='srs' value='Select SRS File' />
                    <FileInput
                        id='srs_vk'
                        name='srs'
                        className='my-4'
                    />
                </div>
                <Button type='submit' color='dark' className='w-full self-center mt-4'>
                    Generate Vk
                </Button>
            </form>
        </div>
    )
}
function GenPkArtifactForm({
    handleSubmit,
    alert,
    warning
}: {
    handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
    alert: string
    warning: string
}) {
    return (
        <div className='flex flex-col'>
            <h1 className='text-2xl mb-6 '>Generate Proving Key</h1>
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
                {/* VK */}
                <div>
                    <Label color="white" htmlFor='vk' value='Select VK File' />
                    <FileInput
                        id='vk'
                        name='vk'
                        className='my-4'
                    />
                </div>
                {/* COMPILED ONNX MODEL */}
                <div>
                    <Label color="white" htmlFor='compiled_onnx' value='Select Compiled Onnx File' />
                    <FileInput
                        id='compiled_onnx_pk'
                        name='compiled_onnx'
                        className='my-4'
                    />
                </div>
                {/* SRS */}
                <div>
                    <Label color="white" htmlFor='srs' value='Select SRS File' />
                    <FileInput
                        id='srs_pk'
                        name='srs'
                        className='my-4'
                    />
                </div>
                <Button type='submit' color='dark' className='w-full self-center mt-4'>
                    Generate Pk
                </Button>
            </form>
        </div>
    )
}

```
===
+++


#### Making a proof
+++ CLI
First we generate a witness file.

```bash
ezkl gen-witness -D input.json -M network.ezkl
```

Next we will generate a proof that the model was correctly run on private inputs (this is the default setting). It then outputs the resulting proof at the path specfifed by `--proof-path`.

```bash
ezkl prove -M network.ezkl --witness witness.json --pk-path=pk.key --proof-path=model.proof --srs-path=kzg.srs
```
+++ Python
To generate a proof, we first need to make a witness file. We can do this by running a forward pass using the input data on the compiled model, saving the output to a witness file specificed by `witness_path`.

We can use this witness, along with the compiled model, proving key and SRS to generate a proof that the model was correctly run on public inputs. It then outputs the resulting proof at the path specfifed by `proof_path`.

Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet.

```python
proof_path = os.path.join('test.pf')
compiled_model_path = os.path.join('network.compiled')
srs_path = os.path.join('kzg.srs')
pk_path = os.path.join('test.pk')
data_path = os.path.join('input.json')
witness_path = os.path.join('witness.json')

# generate witness
res = ezkl.gen_witness(data_path, compiled_model_path, witness_path)
assert os.path.isfile(witness_path)

# generate proof
res = ezkl.prove(
        witness_path,
        compiled_model_path,
        pk_path,
        proof_path,
        srs_path,
        "single",
    )

assert os.path.isfile(proof_path)
```
+++ JS

Use the form rendered below to generate ZKML proofs in the browser right now :)

[!embed el="embed" aspect="1:1" width="720" height="660"](https://ezkljs-engine.vercel.app/prove)
==- View Source Code
```typescript prove.tsx
'use client'
import {
  FileInput,
  Label,
  Button,
  Alert,
  Spinner as _Spinner,
  Modal
} from 'flowbite-react'
import React, { useState } from 'react'
import { formDataSchemaProve } from './parsers'
import { stringify } from "json-bigint";
import { useSharedResources } from '../EngineContext';

export default function Prove() {
  const { engine, utils } = useSharedResources();
  const [openModal, setOpenModal] = useState<string | undefined>();
  const props = { openModal, setOpenModal };
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
    utils.handleGenProofButton(files as { [key: string]: File })
      .then(({ output, executionTime }) => {
        setBuffer(output)



        // Update result based on the outcome
        setProofResult(
          output
            ? `Proof generation successful. Execution time: ${executionTime} ms`
            : "Proof generation failed"
        )
        // Deseralize proof buffer
        // TODO - uncomment this line once a new engine bundle is relased
        // with patch to web based serialize/deserialize methods.
        const proof = engine.deserialize(output)
        console.log("proof", proof)
        setProof(proof);
      })
      .catch((error) => {
        console.error('An error occurred:', error)
        setWarningProof(`Proof generation failed: ${error}`)
      })

    setLoading(false)
  }

  return (
    <div className='flex flex-col justify-center items-center h-5/6 pb-20'>
      {buffer && !warningProof ? (
        <div className='w-10/12 flex flex-col'>
          <h1 className='text-2xl mb-6 '>{proofResult}</h1>
          <div className="flex w-full justify-center pt-7">
            <Button
              className="w-1/2 mr-3"
              type='submit'
              onClick={() => utils.handleFileDownload('test.pf', buffer)}
            >
              Download Proof File
            </Button>
            <Button
              className="w-1/2 mr-3"
              onClick={() => props.setOpenModal('default')}
              data-modal-target="witness-modal"
              data-modal-toggle="witness-modal"
            >
              Show Proof
            </Button>
            <Button
              className="w-1/2"
              onClick={() => setBuffer(null)}
            >
              Reset
            </Button>
            <Modal
              show={props.openModal === 'default'}
              onClose={() => props.setOpenModal(undefined)}
            >
              <Modal.Header>Proof File Content: </Modal.Header>
              <Modal.Body className="bg-black">
                <div className='mt-4 p-4 bg-black-100 rounded border'>
                  <pre className='blackspace-pre-wrap'>{stringify(proof, null, 6)}</pre>
                </div>
              </Modal.Body>
            </Modal>
          </div>
        </div>
      ) : loading ? (
        <Spinner />
      ) : (
        <div className='flex flex-col justify-between w-full items-center space-y-4'>
          <div className='flex justify-between w-full items-stretch space-x-8'>
            <ProvingArtifactForm handleSubmit={handleSubmitProve} alert={alertProof} warning={warningProof} />
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
  );
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
  function assertElement<T extends Element>(element: T | null): asserts element is T {
    if (element === null) {
      throw new Error('Element not found');
    }
  }

  // Names of the sample files in the public directory
  const sampleFileNames: { [key: string]: string } = {
    witness: 'test.witness.json',
    pk: 'test.provekey',
    compiled_onnx: 'test_network.compiled',
    srs: 'kzg'
  };

  // Helper function to fetch and create a file object from a public URL
  const fetchAndCreateFile = async (path: string, filename: string): Promise<File> => {
    const response = await fetch(path);
    const blob = await response.blob();
    return new File([blob], filename, { type: blob.type });
  };

  // Fetch each sample file and create a File object
  const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
    fetchAndCreateFile(`/data/${filename}`, filename)
  );

  // Wait for all files to be fetched and created
  const files = await Promise.all(filePromises);

  // Select the file input elements and assign the FileList to each
  const witness = document.querySelector<HTMLInputElement>('#witness');
  const pk = document.querySelector<HTMLInputElement>('#pk');
  const compiled_onnx = document.querySelector<HTMLInputElement>('#compiled_onnx');
  const srsProve = document.querySelector<HTMLInputElement>('#srs_prove');

  // Assert that the elements are not null
  assertElement(witness);
  assertElement(pk);
  assertElement(compiled_onnx);
  assertElement(srsProve);

  // Create a new DataTransfer to hold the files
  let dataTransfers: DataTransfer[] = [];
  files.forEach(
    (file, idx) => {
      const dataTransfer = new DataTransfer();
      dataTransfer.items.add(file)
      dataTransfers[idx] = dataTransfer;
    }

  );


  witness.files = dataTransfers[0].files;
  pk.files = dataTransfers[1].files;
  compiled_onnx.files = dataTransfers[2].files;
  srsProve.files = dataTransfers[3].files;
}

function ProvingArtifactForm({
  handleSubmit,
  alert,
  warning
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
          <Label color="white" htmlFor='witness' value='Select Witness File' />
          <FileInput
            id='witness'
            name='witness'
            className='my-4'
          />
        </div>
        {/* PK */}
        <div>
          <Label color="white" htmlFor='pk' value='Select Proving Key File' />
          <FileInput
            id='pk'
            name='pk'
            className='my-4'
          />
        </div>
        {/* COMPILED ONNX MODEL */}
        <div>
          <Label color="white" htmlFor='compiled_onnx' value='Select Compiled Onnx File' />
          <FileInput
            id='compiled_onnx'
            name='compiled_onnx'
            className='my-4'
          />
        </div>
        {/* SRS */}
        <div>
          <Label color="white" htmlFor='srs' value='Select SRS File' />
          <FileInput
            id='srs_prove'
            name='srs'
            className='my-4'
          />
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
ezkl verify --proof-path=model.proof --settings-path=settings.json --vk-path=vk.key --srs-path=kzg.srs
```
+++ Python
Using the proof, settings, verification key and SRS, we can verify our proof.

Check out [this colab notebook](https://colab.research.google.com/github/zkonduit/ezkl/blob/main/examples/notebooks/simple_demo_all_public.ipynb) for more context around this code snippet.

```python
proof_path = os.path.join('test.pf')
srs_path = os.path.join('kzg.srs')
settings_path = os.path.join('settings.json')
vk_path = os.path.join('test.vk')

res = ezkl.verify(
        proof_path,
        settings_path,
        vk_path,
        srs_path,
    )

assert res == True
```
+++ JS
Use the form rendered below to verify ZKML proofs in the browser right now :)

[!embed el="embed" aspect="1:1" width="720" height="660"](https://ezkljs-engine.vercel.app/verify)

==- View Source Code
```typescript verify.tsx
// Example for pages/Page1.js
'use client'
import {
    FileInput,
    Label,
    Button,
    Alert,
    Spinner as _Spinner
} from 'flowbite-react'
import React, { useState } from 'react'
import { formDataSchemaVerify } from './parsers'
import { useSharedResources } from '../EngineContext';

export default function Verify() {
    const { utils } = useSharedResources();
    const [alertVerify, setAlertVerify] = useState<string>('')
    const [warningVerify, setWarningVerify] = useState<string>('')
    const [loading, setLoading] = useState(false)
    const [verifyResult, setVerifyResult] = useState<string>('');

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
            srs: validatedFormInputs.data.srs
        }
        /* ================== ENGINE API ====================== */
        utils.handleVerifyButton(files as { [key: string]: File })
            .then(({ output, executionTime }) => {
                // Update result based on the outcome
                setVerifyResult(
                    output
                        ? 'Verification successful. Execution time: ' + executionTime + ' ms'
                        : 'Verification failed'
                )
            })
            .catch((error) => {
                console.error('An error occurred:', error)
                setWarningVerify(`Verification process failed with an error: ${error}`)
            })

        setLoading(false)
    }


    return (
        <div className='flex flex-col justify-center items-center h-5/6 pb-20'>
            {verifyResult && !warningVerify ? (
                <div className='w-10/12 flex flex-col'>
                    <h1 className='text-2xl mb-6 '>{verifyResult}</h1>
                    <div className="flex w-full justify-center">
                        <Button
                            className="w-full"
                            onClick={() => setVerifyResult("")}
                        >
                            Reset
                        </Button>
                    </div>
                </div>
            ) : loading ? (
                <Spinner />
            ) : (
                <div className='flex flex-col justify-between w-full items-center space-y-4'>
                    <div className='flex justify-between w-full items-stretch space-x-8'>
                        <VerifyingArtifactForm handleSubmit={handleSubmitVerify} alert={alertVerify} warning={warningVerify} />
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
    );
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
    function assertElement<T extends Element>(element: T | null): asserts element is T {
        if (element === null) {
            throw new Error('Element not found');
        }
    }

    // Names of the sample files in the public directory
    const sampleFileNames: { [key: string]: string } = {
        srs: 'kzg',
        proof: 'test.pf',
        settings: 'settings.json',
        vk: 'test.key'
    };

    // Helper function to fetch and create a file object from a public URL
    const fetchAndCreateFile = async (path: string, filename: string): Promise<File> => {
        const response = await fetch(path);
        const blob = await response.blob();
        return new File([blob], filename, { type: blob.type });
    };

    // Fetch each sample file and create a File object
    const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
        fetchAndCreateFile(`/data/${filename}`, filename)
    );

    // Wait for all files to be fetched and created
    const files = await Promise.all(filePromises);

    // Select the file input elements and assign the FileList to each
    const proof = document.querySelector<HTMLInputElement>('#proof');
    const settings = document.querySelector<HTMLInputElement>('#settings');
    const vk = document.querySelector<HTMLInputElement>('#vk');
    const srsVerify = document.querySelector<HTMLInputElement>('#srs_verify');

    // Assert that the elements are not null
    assertElement(proof);
    assertElement(settings);
    assertElement(vk);
    assertElement(srsVerify);

    // Create a new DataTransfer to hold the files
    let dataTransfers: DataTransfer[] = [];
    files.forEach(
        (file, idx) => {
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(file)
            dataTransfers[idx] = dataTransfer;
        }

    );

    srsVerify.files = dataTransfers[0].files;
    proof.files = dataTransfers[1].files;
    settings.files = dataTransfers[2].files;
    vk.files = dataTransfers[3].files;
}

function VerifyingArtifactForm({
    handleSubmit,
    alert,
    warning
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
                className='flex flex-col flex-grow  justify-between'
            >
                {/* PROOF */}
                <div>
                    <Label color="white" htmlFor='proof' value='Select Proof File' />
                    <FileInput
                        id='proof'
                        name='proof'
                        className='my-4'
                    />
                </div>
                {/* SETTINGS */}
                <div>
                    <Label color="white" htmlFor='settings' value='Select Settings File' />
                    <FileInput
                        id='settings'
                        name='settings'
                        className='my-4'
                    />
                </div>
                {/* VK */}
                <div>
                    <Label color="white" htmlFor='vk' value='Select VK File' />
                    <FileInput
                        id='vk'
                        name='vk'
                        className='my-4'
                    />
                </div>
                {/* SRS */}
                <div>
                    <Label color="white" htmlFor='srs' value='Select SRS File' />
                    <FileInput
                        id='srs_verify'
                        name='srs'
                        className='my-4'
                    />
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

[!embed el="embed" aspect="1:1" width="720" height="660"](https://ezkljs-engine.vercel.app/in-browser-evm-verify)

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
import { useSharedResources } from '../EngineContext';

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
    const { utils } = useSharedResources();
    const [alertVerify, setAlertVerify] = useState<string>('')
    const [warningVerify, setWarningVerify] = useState<string>('')
    const [loading, setLoading] = useState(false)
    const [evmVerifyResult, setEvmVerifyResult] = useState<string>('');

    const handleSubmitVerify = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const formData = new FormData(e.currentTarget)

        const formInputs = {
            proof: formData.get('proof'),
            bytecode_verifier: formData.get('bytecode_verifier'),
            evm_version: formData.get('evm_version')
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
            bytecodeVerifier: validatedFormInputs.data.bytecode_verifier
        }
        /* ================== ENGINE API ====================== */
        utils.handleEvmVerifyButton(files as { [key: string]: File }, validatedFormInputs.data.evm_version as Hardfork)
            .then(({ output, executionTime }) => {
                // Update result based on the outcome
                setEvmVerifyResult(
                    output
                        ? 'In-browser EVM verification successful. Execution time: ' + executionTime + ' ms'
                        : 'In-browser EVM verification failed'
                )
            })
            .catch(error => {
                console.error("An error occurred:", error);
                setEvmVerifyResult("An error occurred: " + error);
            })

        setLoading(false)
    }


    return (
        <div className='flex flex-col justify-center items-center h-5/6 pb-20'>
            {evmVerifyResult && !warningVerify ? (
                <div className='w-10/12 flex flex-col'>
                    <h1 className='text-2xl mb-6 '>{evmVerifyResult}</h1>
                    <div className="flex w-full justify-center">
                        <Button
                            className="w-1/2"
                            onClick={() => setEvmVerifyResult("")}
                        >
                            Reset
                        </Button>
                    </div>
                </div>
            ) : loading ? (
                <Spinner />
            ) : (
                <div className='flex flex-col justify-between w-full items-center space-y-4'>
                    <VerifyingArtifactForm handleSubmit={handleSubmitVerify} alert={alertVerify} warning={warningVerify} />
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
    );
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
    function assertElement<T extends Element>(element: T | null): asserts element is T {
        if (element === null) {
            throw new Error('Element not found');
        }
    }

    // Names of the sample files in the public directory
    const sampleFileNames: { [key: string]: string } = {
        proof: 'evm_verify.pf',
        bytecode: 'bytecode.code'
    };

    // Helper function to fetch and create a file object from a public URL
    const fetchAndCreateFile = async (path: string, filename: string): Promise<File> => {
        const response = await fetch(path);
        const blob = await response.blob();
        return new File([blob], filename, { type: blob.type });
    };

    // Fetch each sample file and create a File object
    const filePromises = Object.entries(sampleFileNames).map(([key, filename]) =>
        fetchAndCreateFile(`/data/${filename}`, filename)
    );

    // Wait for all files to be fetched and created
    const files = await Promise.all(filePromises);

    // Select the file input elements and assign the FileList to each
    const proof = document.querySelector<HTMLInputElement>('#proof');
    const bytecode = document.querySelector<HTMLInputElement>('#bytecode_verifier');

    // Assert that the elements are not null
    assertElement(proof);
    assertElement(bytecode);

    // Create a new DataTransfer to hold the files
    let dataTransfers: DataTransfer[] = [];
    files.forEach(
        (file, idx) => {
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(file)
            dataTransfers[idx] = dataTransfer;
        }

    );


    proof.files = dataTransfers[0].files;
    bytecode.files = dataTransfers[1].files;

    // // If the 'vk' file is different, you'd handle it separately
    // const vkFile = await fetchAndCreateFile(`/${sampleFileNames.vk}`, sampleFileNames.vk);
    // const vkDataTransfer = new DataTransfer();
    // vkDataTransfer.items.add(vkFile);

    // Trigger any onChange or update logic if necessary
    // This part depends on your application. For example, you might need to call a state setter function if you're using React state to track file input values.
}

function VerifyingArtifactForm({
    handleSubmit,
    alert,
    warning
}: {
    handleSubmit: (e: React.FormEvent<HTMLFormElement>) => void
    alert: string
    warning: string
}) {
    const [selectedVersion, setSelectedVersion] = useState<Hardfork>(Hardfork.Istanbul);

    const handleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const selectedValue = e.target.value as Hardfork;
        setSelectedVersion(selectedValue);

    };
    return (
        <div className='flex flex-col' >
            <h1 className='text-2xl mb-6 '>In-Browser Evm Verifying</h1>
            {
                alert && (
                    <Alert color='info' className='mb-6'>
                        {alert}
                    </Alert>
                )
            }
            {
                warning && (
                    <Alert color='warning' className='mb-6'>
                        {warning}
                    </Alert>
                )
            }
            <form
                onSubmit={handleSubmit}
                className='flex flex-col flex-grow  justify-between'
            >
                {/* PROOF */}
                <div>
                    <Label color="white" htmlFor='proof' value='Select Proof File' />
                    <FileInput
                        id='proof'
                        name='proof'
                        className='my-4'
                    />
                </div>
                {/* VERIFIER BYTECODE */}
                <div>
                    <Label color="white" htmlFor='bytecode_verifier' value='Select Evm Verifier Bytecode' />
                    <FileInput
                        id='bytecode_verifier'
                        name='bytecode_verifier'
                        className='my-4'
                    />
                </div>
                {/* EVM VERSION */}
                <div>
                    <Label color="white" htmlFor='evm_version' value='Select Evm Version' />
                    <Select
                        id='evm_version'
                        name='evm_version'
                        className='my-4'
                        onChange={handleChange}
                        value={selectedVersion}
                    >
                        {Object.keys(Hardfork).map(key => (
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
        </div >
    )
}

```
===
+++



