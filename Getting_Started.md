---
icon: code-square
order: 93
---
![](../assets/cli.png) 
The `ezkl` cli provides a simple interface to load `.onnx` files, which represent graphs of operations (such as neural networks), convert them, and run a proof.

## Tutorial 👾

You can easily create an `.onnx` file using `pytorch`. For samples of Onnx files see [here](https://github.com/onnx/models). To see how to generate Onnx files using python, check out <a href="https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/" target="_blank"> the notebooks.</a>. You'll also need an `input.json` file with sample inputs and outputs of your model.

Sample onnx files are also available in <a href="https://github.com/zkonduit/ezkl/blob/main/examples/onnx/" target="_blank"> the repo </a>.

#### Initializing the project
To generate a proof on one of the examples, first install `ezkl` 
[!ref](/getting_started)

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
+++ EZKL Engine
For performance reaons, you can only generate settings using the hub, python and cli environments. Stay tuned for updates!
+++ HUB Api
In order to query the artifacts currently available on the EZKL Hub you can use the `getArtifacts` method.

This method accepts an options object which allows you to specify the `limit` (the max number of artifacts to return) and `skip` (the number of artifacts to skip). `skip` and `limit` can be used together for effective pagination. If no options are provided, the default values are `skip = 0` and `limit = 20`.

```typescript
type Artifact = {
  name: string
  description: string
  id: string
}

type PageOptions =
  | {
      skip?: number
      limit?: number
    }
  | undefined

const pageOptions: PageOptions = {
  skip: 0,
  limit: 2,
  url: 'https://hub.ezkl.xyz',
}

const artifacts: Artifact[] = await hub.getArtifacts(pageOptions)

console.log(JSON.stringify(artifacts), null, 2)
```

Output:

```json
[
  {
    "name": "test",
    "description": "test",
    "id": "b7000626-ed7a-418c-bcf1-ccd10661855a"
  },
  {
    "name": "test",
    "description": "test",
    "id": "e7e92ecf-f020-4603-a908-4b40b7846874"
  }
]
```

Once you have an artifact, you can fetch the settings file using the hub's [graphql endpoint](https://hub.ezkl.xyz/graphql). Here is an example of how to get a set of the first 20 settings files created by a particular organization ("My Organizaiton") on the Hub. It also shows you how to download the first settings file using the settings endpoint.

```typescript
function buildArtifactsQuery(
  orgName: string,
  skip: number,
  first: number,
) {
// Here we fetch the first 20 artifacts created by a particular organization on the Hub, getting back the id and compiled mode download endpoint for each artifact.
  return `query Artifacts  {
      artifacts (
        organizationName: "${orgName}", skip: ${skip}, first: ${first}
        orderBy: {field: "createdAt", order: ASC}
      ) {
        id
        settings
      }
    }`;
}

async function fetchArtifacts(orgName: string, skip: number) {
  const query = buildArtifactsQuery(orgName, skip, 20);
  try {
    const resp = await fetch("https://hub.ezkl.xyz/graphql", {
      cache: "no-store",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        query,
      }),
    });
    const artifactsJsonResp = await resp.json();
    const artifacts: Artifacts = artifactsSchema.parse(
      artifactsJsonResp?.data?.artifacts,
    );

    return artifacts;
  } catch (e) {
    throw e;
  }
}

async function downloadSettings(settingsEndpoint: string) {
    const settingsUrl = `https://hub.ezkl.xyz/download/${settingsEndpoint}`;

    const settingsResp = await fetch(settingsUrl);

    // Check if the request was successful
    if (!settingsResp.ok) {
        throw new Error('Failed to fetch the model.');
    }

    const blob = await settingsResp.blob();

    // Create a link element and trigger a download
    const link = document.createElement('a');
    link.href = URL.createObjectURL(blob);
    link.download = 'compiled_model'; 
    document.body.appendChild(link); 
    link.click();
    document.body.removeChild(link); 
}


fetchArtifacts("My Organization", 0).then((artifacts) => {
    // log the artifacts to the console
    console.log(JSON.stringify(artifacts, null, 2));
    // download the first settings file
    downloadModel(artifacts[0].settings);
});
```
Output:

```json
[{
  "id": "6017cb49-cdb8-4648-9422-c8568de9a2f5",
  "settings": "6017cb49-cdb8-4648-9422-c8568de9a2f5/settings.json"
}]
```
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
+++ EZKL Engine
For performance reaons, you can only compile ONNX models using the hub, python and cli environments. Stay tuned for updates!
+++ HUB Api
With EZKL Hub, you can upload an uncompiled `ONNX` model and let Hub compile your circuit with default settings. You can do this by using the `genArtifact` method.

In order to upload an artifact you'll need to provide the following:

1. `name`: The name of your artifact.
2. `description`: A description of your artifact.
3. `organizationId`: The organization you wish to upload your artifact to.
4. `modelFile`: The model you wish to upload in `.onnx` format.
5. `inputFile`: A representative input file in JSON format.
6. `url` you can provide an optional url if you're using a custom EZKL Hub instance

This will work with either in a browser client (`File`) or a Node.js (`Buffer`) environent.

```typescript
const name: string = 'My Artifact Name'
const description: string = 'My Artifact Description'
const organizationId: string = 'b7000626-ed7a-418c-bcf1-ccd10661855a' // uuid
const modelFile: File | Buffer = fs.readFileSync('/path/model.onnx')
const inputFile: File | Buffer = fs.readFileSync('/path/input.json')
const url: string = 'https://hub.ezkl.xyz'

const genArtifactResponse = await hub.genArtifact({
  name,
  description,
  organizationId,
  url,
  modelFile,
  inputFile,
  url,
})

console.log(JSON.stringify(genArtifactResponse), null, 2)
```

Output:

```json
{
  "id": "6017cb49-cdb8-4648-9422-c8568de9a2f5"
}
```

Once you have an artifact, you can fetch the compiled model using the hub's [graphql endpoint](https://hub.ezkl.xyz/graphql). Here is an example of how to get a set of the first 20 compiled models created by a particular organization ("My Organizaiton") on the Hub. It also shows you how to download the first model using the compiled model endpoint.

```typescript
function buildArtifactsQuery(
  orgName: string,
  skip: number,
  first: number,
) {
// Here we fetch the first 20 artifacts created by a particular organization on the Hub, getting back the id and compiled mode download endpoint for each artifact.
  return `query Artifacts  {
      artifacts (
        organizationName: "${orgName}", skip: ${skip}, first: ${first}
        orderBy: {field: "createdAt", order: ASC}
      ) {
        id
        model
      }
    }`;
}

async function fetchArtifacts(orgName: string, skip: number) {
  const query = buildArtifactsQuery(orgName, skip, 20);
  try {
    const resp = await fetch("https://hub.ezkl.xyz/graphql", {
      cache: "no-store",
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        query,
      }),
    });
    const artifactsJsonResp = await resp.json();
    const artifacts: Artifacts = artifactsSchema.parse(
      artifactsJsonResp?.data?.artifacts,
    );

    return artifacts;
  } catch (e) {
    throw e;
  }
}

async function downloadModel(compiledModelEndpoint: string) {
    const modelUrl = `https://hub.ezkl.xyz/download/${compiledModelEndpoint}`;

    const modelResp = await fetch(modelUrl);

    // Check if the request was successful
    if (!modelResp.ok) {
        throw new Error('Failed to fetch the model.');
    }

    const blob = await modelResp.blob();

    // Create a link element and trigger a download
    const link = document.createElement('a');
    link.href = URL.createObjectURL(blob);
    link.download = 'compiled_model'; 
    document.body.appendChild(link); 
    link.click();
    document.body.removeChild(link); 
}


fetchArtifacts("My Organization", 0).then((artifacts) => {
    // log the artifacts to the console
    console.log(JSON.stringify(artifacts, null, 2));
    // download the first model
    downloadModel(artifacts[0].model);
});
```
Output:

```json
[{
  "id": "6017cb49-cdb8-4648-9422-c8568de9a2f5",
  "model": "6017cb49-cdb8-4648-9422-c8568de9a2f5/network.compiled"
}]
```
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



#### Making a proof
First we generate a witness file.

```bash
ezkl gen-witness -D input.json -M network.ezkl
```

Next we will generate a proof that the model was correctly run on private inputs (this is the default setting). It then outputs the resulting proof at the path specfifed by `--proof-path`.

```bash
ezkl prove -M network.ezkl --witness witness.json --pk-path=pk.key --proof-path=model.proof --srs-path=kzg.srs
```

#### Verification
We can then verify our generated proof with the `verify` command:
```bash
ezkl verify --proof-path=model.proof --settings-path=settings.json --vk-path=vk.key --srs-path=kzg.srs
```

#### All together
```bash
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/network.onnx ./
cp ~/ezkl/examples/onnx/4l_relu_conv_fc/input.json ./
ezkl gen-settings -M network.onnx
ezkl calibrate-settings -M network.onnx -D input.json --target resources
ezkl get-srs -S settings.json
ezkl compile-circuit -M network.onnx -S settings.json --compiled-circuit network.ezkl
ezkl setup -M network.ezkl --srs-path=kzg.srs --vk-path=vk.key --pk-path=pk.key
ezkl gen-witness -D input.json -M network.ezkl
ezkl prove -M network.ezkl --witness witness.json --pk-path=pk.key --proof-path=model.proof --srs-path=kzg.srs
ezkl verify --proof-path=model.proof --settings-path=settings.json --vk-path=vk.key --srs-path=kzg.srs
```



