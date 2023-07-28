---
order: 3
---


# Idol Model Tutorial


### Background Knowledge

Check out these links for some useful knowledge on: 

- [EVM](https://ethereum.org/en/developers/docs/evm/),
- [PyTorch](https://pytorch.org/tutorials/beginner/deep_learning_60min_blitz.html), and
- [zero knowledge proof cryptography](https://en.wikipedia.org/wiki/Zero-knowledge_proof).

### Voice data

The voice datasets we will use are labeled using the same emotion and tone labeling standard and consist of 8 emotions: neutral, calm, happy, sad, angry, fear, disgust, surprise. The datasets are obtained from `Kaggle` and include the `TESS, RAVDESS SONG, RAVDESS SPEECH, CREMA`, and `SAVEE` datasets.

Here we showcase a full-end-to-end flow of:

1. training a model for a specific task (judging voices)
2. creating a proof of judgment
3. creating and deploying an evm verifier
4. verifying the proof of judgment using the verifier

First we download a few voice related datasets from kaggle, which are all labelled using the same emotion and tone labelling standard.

We have 8 emotions in both speaking and singing datasets: neutral, calm, happy, sad, angry, fear, disgust, surprise.

To download the dataset make sure you have the kaggle cli installed in your local env `pip install kaggle`. Make sure you set up your `kaggle.json` file as detailed [here](https://www.kaggle.com/docs/api#getting-started-installation-&-authentication). Then run the associated `voice_data.sh` [data download](https://github.com/zkonduit/ezkl/blob/main/examples/notebooks/voice_data.sh) script: `sh voice_data.sh`

Make sure you set the `VOICE_DATA_DIR` variables to point to the directory the `voice_data.sh` script has downloaded to. This script also accepts an argument to download to a specific directory: `sh voice_data.sh /path/to/voice/data`.

### Training

During training we convert all audio files into 2D frequency-domain spectrograms so that we can leverage convolutional neural networks, which tend to be more efficient than time-series model like RNNs or LSTMs. We thus:

1. Extract the mel spectrogram from each of the audio recordings.
2. Rescale each of these to the decibel (DB) scale.
3. Define the model as the following model:
`(x) -> (conv) -> (relu) -> (linear) -> (y)`

You may notice that we introduce a second computational graph `(key) -> (key)`. The reasons for this are to prevent someone else from stealing your submission, and if you are not interested you can skip the following paragraph.

### MEV prevention

Let's say that obtaining a high score from the judge and then submitting said score to the EVM verifier could result in the issuance of a reward (financial or otherwise). There is an incentive then for MEV bots to scalp any issued valid proof and submit a duplicate transaction with the same proof to the verifier contract in the hopes of obtaining the reward before the original issuer. Here we add `(key) -> (key)` such that the transaction creator's public key / address is both a private input AND a public input to the proof. As such the on-chain verification only succeeds if the key passed in during proof time is also passed in as a public input to the contract. The reward issued by the contract can then be irrevocably tied to that key such that even if the proof is submitted by another actor, the reward would STILL go to the original singer / transaction issuer.

We leverage the often-used Adam optimizer, coupled with 0.001 weight decay so as to regularize the model. The weight decay (a.k.a L2 regularization) can also help on the zk-circuit end of things in that it prevents inputs to Halo2 lookup tables from falling out of range (lookup tables are how we represent non-linearities like ReLU and Sigmoid inside our circuits).

To encode the judge’s “preferences”, we convert labels to a number between 0 and 1 where 1 is pleasantly surprised and 0 is disgust and the rest are floats in between. The model loves pleasantly surprised voices and hates disgust ;) . 

### **Exporting and Verifying the Model**

After training the model, we export it to the ONNX format, which is a platform-agnostic format for machine learning models. We also save a sample input to a JSON file for later use.

Next, we generate a settings file for our model using the **`gen_settings`** and **`calibrate_settings`** functions from the EZKL library. This settings file basically instantiates a bunch of parameters that determine the circuit shape, size . We use recommend also running **`calibrate_settings`** because of the way we represent nonlinearities in the circuit (using Halo2's [lookup tables](https://zcash.github.io/halo2/design/proving-system/lookup.html)), it is often best to *calibrate* this settings file as some data can fall out of range of these lookups.

As we use Halo2 with KZG-commitments we need an SRS string from (preferably) a multi-party trusted setup ceremony. For an overview of the procedures for such a ceremony check out [this page](https://blog.ethereum.org/2023/01/16/announcing-kzg-ceremony). The`get_srs` command retrieves a correctly sized SRS given the calibrated settings file from [here](https://github.com/han0110/halo2-kzg-srs).

We then generate a witness for our model, which are the model outputs when feeding the previously saved input through the model. After that, we run a mock proof to check that all the constraints are valid.

Next, we set up the proving and verifying keys for our model and generate a full proof. We then verify the proof as a sanity check.

**Deploying and Verifying the EVM Verifier**

Finally, we create an Ethereum Smart Contract that acts as a verifier for our model. We deploy the contract (using `deploy-verifier`) to a local Ethereum node using the [Anvil Ethereum node simulator](https://github.com/foundry-rs/foundry/blob/master/anvil/README.md) running on port `3030`. In a separate terminal window (or using the notebook bash tooling `!`) run `anvil -p 3030` to start the node.

After deploying the contract, we obtain its address which we will use to interact with  and we can then verify the proof using the deployed contract by calling the **`verify_evm`** function from the EZKL library and passing the proof and the contract's address. If everything is set up correctly, the proof should be verified successfully!