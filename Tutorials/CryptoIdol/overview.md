---
order: 5
---

# Overview

# **Building a Voice Emotion Classifier and Verifier with PyTorch, EZKL, and Ethereum Smart Contracts**


This is part 1 of our tutorial on building the [Cryptoidol](https://cryptoidol.tech) demo app. The finished app is on Github; check out the [backend](https://github.com/zkonduit/cryptoidol) and [frontend](https://github.com/zkonduit/cryptoidol-frontend).

## Background Knowledge

This article assumes prior rudimentary knowledge on the [EVM](https://ethereum.org/en/developers/docs/evm/), [PyTorch](https://pytorch.org/tutorials/beginner/deep_learning_60min_blitz.html) and [zero knowledge proof cryptography](https://en.wikipedia.org/wiki/Zero-knowledge_proof).  If you are unfamiliar with one, a few or all of these technologies/methods (or you just want a refresher), check out the linked articles as needed. 

## Introduction

In this tutorial, we will demonstrate an end-to-end flow of training a model for a specific task, creating a proof of judgement, deploying an EVM verifier, and verifying the proof of judgement using the verifier. Specifically, our task is to build a classifier that can judge voices based on their emotional content.

The voice datasets we will use are labeled using the same emotion and tone labeling standard and consist of 8 emotions: neutral, calm, happy, sad, angry, fear, disgust, surprise. The datasets are obtained from Kaggle and include the TESS, RAVDESS SONG, RAVDESS SPEECH, CREMA, and SAVEE datasets.

The code and instructions in this tutorial are provided in a Jupyter notebook which can be run in a Python environment with the necessary libraries installed.

## **Data Preparation**

First, we download the datasets using the [Kaggle](https://www.kaggle.com/datasets) CLI and store them in a directory specified by the **`VOICE_DATA_DIR`** environment variable. We then load the datasets and create a pandas [DataFrame](https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.html) for each. Each DataFrame includes two columns: 'Emotions', which is the label, and 'Files', which is the path to the audio file.

After loading all the datasets, we concatenate them into one DataFrame, which we will use for training our model. We also plot the distribution of emotions in our dataset using [seaborn](https://seaborn.pydata.org/) to visualize the data.

## **Training**

We will train a Convolutional Neural Network (CNN) model using PyTorch to classify the voice data. We choose a CNN model because we will convert all audio files into 2D frequency-domain spectrograms, which CNNs are well-suited to handle.

Our model is defined with a Conv2d layer followed by a ReLU activation function and a Linear layer. We then train the model using the Adam optimizer with a learning rate of 0.001 and a weight decay of 0.001 for regularization. We use Mean Squared Error as the loss function.

The model is trained for 10 epochs with a batch size of 10. We also split the data into training, validation, and test sets with 80%, 10%, and 10% of the data, respectively. After each epoch, we compute the validation loss to monitor the model's performance on unseen data.

## **Exporting and Verifying the Model**

After training the model, we export it to the ONNX format, which is a platform-agnostic format for machine learning models. We also save a sample input to a JSON file for later use.

Next, we generate a settings file for our model using the **`gen_settings`** and **`calibrate_settings`** functions from the EZKL library. We also download a Structured Reference String (SRS) which is needed for generating Zero Knowledge Proofs (ZKPs).

We then generate a witness for our model, which are the model outputs when feeding the previously saved input through the model. After that, we run a mock proof to check that all the constraints are valid.

Next, we set up the proving and verifying keys for our model and generate a full proof. We then verify the proof as a sanity check.

## **Deploying and Verifying the EVM Verifier**

Finally, we create an Ethereum Smart Contract that acts as a verifier for our model. We deploy the contract to a local Ethereum node using the Anvil Ethereum node simulator. After deploying the contract, we obtain its address which we will use to interact with it.

We then verify the proof using the deployed contract by calling the **`verify_evm`** function from the EZKL library and passing the proof and the contract's address. If everything is set up correctly, the proof should be verified successfully.

## **Conclusion**

We hope this tutorial provides a foundation for building more complex production-ready application on EZKL that require secure, verifiable judgments.
