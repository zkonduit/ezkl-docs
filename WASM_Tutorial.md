**Insert space renaissance image**

#### Getting Started

It is useful to have a verifier on a blockchain. However, sometimes you simply want to generate and verify proofs in the browser. Thankfully, ezkl supports a WASM environment that you can use to generate proofs and verify them in-browser. For those who are unfamiliar, [**here**](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts) is a good resource on WASM and [**here**](https://github.com/zkonduit/ezkl/blob/main/src/wasm.rs) you can find the functions we define for ezkl's WASM interface. Let's get started!

First, we need to add the `wasm32-unknown-unknown` target to our rustup configuration.  `wasm32-unknown-unknown` is is a target for creating WebAssembly binaries. The `wasm32` part represents the platform (WASM 32 bit in our case). The first `unknown` specifies the operating system we are building on. We want to build on any operating system since we're just building on browser. The second `unknown` refers to the target's standard library (Rust/C++ `std`), but with WASM, we won't be using one. We add this as a target with:

> rustup target add wasm32-unknown-unknown

Another thing we need before we get our `.wasm` file is [LLVM](https://llvm.org/). LLVM is a compiler tool that will help us use libraries that are essential for compiling our Rust ezkl code to a WASM binary fit for `wasm32-unknown-unknown`. You can get the latest release [here](https://releases.llvm.org/download.html) (especially for Windows users) or install it with a package manager:

*Linux*

>```py
>sudo apt install llvm
>sudo apt install clang-12
>```

*Mac*: You can use Homebrew to install llvm. This library comes with `clang`, which we'll also need.

> ```shell
> brew install --debug llvm
> export PATH=/usr/local/opt/llvm/bin:$PATH
> ```

After this step, make sure you have access to the `PATH` for both `clang` and `llvm`. We'll be using environment variables such as `CC=/opt/homebrew/opt/llvm/bin/clang` for the remainder of the project. 

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/book/)

> ```
> curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
> ```

Now, navigate to your fork or branch of `ezkl` and install the [WASM server runner](https://crates.io/crates/wasm-server-runner):

> ```shell
> cargo install wasm-server-runner
> ```

Also note that you should be on Rust's nightly release channel when interacting with ezkl. And with this, we're finally able to compile our .wasm file! You can do that with this command:

> ```shell
> RUSTFLAGS='-C target-feature=+atomics,+mutable-globals,+bulk-memory' AR=/usr/local/opt/llvm/bin/llvm-ar  CC=/usr/local/opt/llvm/bin/clang wasm-pack build --target wasm32-unknown-unknown -Z build-std="panic_abort,std"
> ```

###### Note for internal: We should look into the errors that are thrown when the `--release` flag is used

##### Note: if you experience errors with wasm-pack, you can use `cargo` instead. Just know that you will need to create a `pkg` directory later for your WASM package. 

The `RUSTFLAGS` are used for extra formatting when compiling the WASM binaries. We also need access to our `llvm` and `clang` instances. The `-Z` flag is used to include the Rust standard library in our build and to abort when panic occurs. You can also run prefabricated tests with:

```shell
RUSTFLAGS='-C target-feature=+atomics,+mutable-globals,+bulk-memory' AR=/usr/local/opt/llvm/bin/llvm-ar  CC=/usr/local/opt/llvm/bin/clang wasm-pack test --firefox -Z build-std="panic_abort,std"
```



This step should create a `wasm32-unknown-unknown` folder in your `target` directory with an `ezkl.wasm` and an `ezkl_lib.wasm` along with other files. We'll need those and the `.d` files for our npm project. Once you have everything built, it's time to create a frontend that will access our WASM functions!

**Insert "it's WASM time" meme**

#### Configuring and publishing our WASM package

Open a new directory where you'd like your project to be. Make sure you have Node.js installed and are on a stable version of node. We used version `16.20.0`. 

```shell
nvm install 16
nvm use 16
```

Use the command:

```shell
npm init wasm-app my-new-wasm-app
```

to create your WASM project and replace `my-new-wasm-app` with your project's name. This will create a new project with `index.js`, `index.html`, `bootstrap.js`, `package.json`, and more files for the foundation of your project. If you explore the repo, you'll see that there is a default `hello-wasm-pack` dependency in `package.json`. We want to use our `ezkl` project instead, so you can go ahead and remove that dependency. Also feel free to check out the `wasm-pack` [docs](https://rustwasm.github.io/wasm-pack/book/introduction.html) for more information on building with WASM and Rust. 

Before we build the frontend, let's publish our npm package with our `ezkl` WASM binaries. If you used `wasm-pack` earlier to build the project and you got no errors, great! You can skip to the part where we create an npm account. If you did generate errors with `wasm-pack` and ended up using `cargo` to build the `wasm32-unknown-unknown` binaries, no worries, this part is for you.

Create a `pkg` directory in your Javascript project. You'll need four files there: `ezkl.wasm`, `ezkl.d`, `package.json`, and a `.js` for the function exports (we used `ezkl.js`). Go ahead and copy over `ezkl.wasm` & `ezkl.d` from your `wasm32-unknown-unknown` package. Then, create a new `package.json` file with this code:

```json
{
  "name": "ezkl-lib-wasm-test",
  "version": "1.0.0",
  "description": "Wasm module for the ezkl zkml library",
  "main": "ezkl.js",
  "types": "ezkl.d",
  "files": [
    "ezkl.wasm",
    "ezkl.js",
    "ezkl.d"
  ],
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [
    "wasm",
    "zk-snarks",
    "proofs",
    "verification"
  ],
  "author": "lancenonce",
  "license": "MIT",
  "dependencies": {}
}
```

Change the name of your project to the package name you want along with the description and main Javascript file.  change the "author" to whatever your name / alias is. If your `.d` and `.wasm` names are the same, feel free to leave that alone. Also, change the name of the author of the package to your name / alias. Next, create your main javascript file with these exports:

```javascript
import * as wasm from './ezkl_lib.wasm';

export async function init_panic_hook() {
  return wasm.init_panic_hook();
}

export async function prove_wasm(data, pk, circuit_ser, circuit_params_ser, params_ser) {
  return wasm.prove_wasm(data, pk, circuit_ser, circuit_params_ser, params_ser);
}

export async function verify_wasm(proof, vk, circuit_params_ser, params_ser) {
  return wasm.verify_wasm(proof, vk, circuit_params_ser, params_ser);
}
```

Remember, we are only exporting the `prove_wasm` and `verify_wasm` functions to WASM, not the full `ezkl` CLI. Make sure to save your files!

Create an account with npm (if you don't already have one) and use your username to run this script:

> ```shell
> $ wasm-pack build --scope my-npm-username
> ```

Then, log into npm through `wasm-pack` with:

> ```shell
> $ wasm-pack login
> ```

Finally, change directories to your `pkg` folder and publish your package to your npm profile:

> ```shell
> $ cd pkg
> $ npm publish --access=public
> ```

Great! Now, if you navigate to your npm dashboard, you should see your wasm package generated. It should look something like this:

**Insert picture of npm dashboard**

#### Creating a frontend

Now that our package is published and ready to use, we'll create our frontend. In our context, we'll be using the ezkl library to pass in **input data**, **the proving key**, **the serialized circuit**, **the serialized circuit parameters**, and our **polynomial commitment scheme paramenters** to our `prove_wasm` function. Additionally, we will pass the **proof**, **the verify key**, **the serialized circuit parameters**, and the **polynomial commitment scheme parameters** to our `verify_wasm` function. It is important to note that you will have a lot of this information after you create a circuit with ezkl. Feel free to store it in your .env file. For now, we will create a simple frontend where those values are pasted as text inputs. Let's begin.

We'll first edit our input.js file to take in text inputs and call the `prove_wasm` function with the text as parameters. We'll paste the Result and allow the same for the `verify_wasm` function.

```javascript
import * as wasm from "ezkl-wasm-test2";

window.prove = async () => {
    try {
        const data = document.getElementById("data").value;
        const pk = document.getElementById("pk").value;
        const circuit_ser = document.getElementById("circuit_ser").value;
        const circuit_params_ser = document.getElementById("circuit_params_ser").value;
        const params_ser = document.getElementById("params_ser").value;
        
        const result = await wasm.prove_wasm(data, pk, circuit_ser, circuit_params_ser, params_ser);
        document.getElementById("proveResult").innerText = result;
    } catch (error) {
        console.error('An error occurred:', error);
        alert('An error occurred. Please check the console for more details.');
    }
};

window.verify = async () => {
    try {
        const proof_js = document.getElementById("proof_js").value;
        const vk = document.getElementById("vk").value;
        const circuit_params_ser = document.getElementById("circuit_params_ser_verify").value;
        const params_ser = document.getElementById("params_ser_verify").value;
        
        const result = await wasm.verify_wasm(proof_js, vk, circuit_params_ser, params_ser);
        document.getElementById("verifyResult").innerText = result ? 'True' : 'False';
    } catch (error) {
        console.error('An error occurred:', error);
        alert('An error occurred. Please check the console for more details.');
    }
};
```

Now, we'll edit our `index.html` file to create a UI:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>ezkl WASM Interface</title>
    </head>
    <body>
        <div>
            <h1>Prove</h1>
            <input id="data" type="text" placeholder="data" />
            <input id="pk" type="text" placeholder="pk" />
            <input id="circuit_ser" type="text" placeholder="circuit_ser" />
            <input id="circuit_params_ser" type="text" placeholder="circuit_params_ser" />
            <input id="params_ser" type="text" placeholder="params_ser" />
            <button onclick="prove()">Prove</button>
            <h2>Result:</h2>
            <div id="proveResult"></div>
        </div>
        <div>
            <h1>Verify</h1>
            <input id="proof_js" type="text" placeholder="proof_js" />
            <input id="vk" type="text" placeholder="vk" />
            <input id="circuit_params_ser_verify" type="text" placeholder="circuit_params_ser" />
            <input id="params_ser_verify" type="text" placeholder="params_ser" />
            <button onclick="verify()">Verify</button>
            <h2>Result:</h2>
            <div id="verifyResult"></div>
        </div>
        <script src="index.js"></script>
    </body>
</html>
```

You'll also notice you still have a default webpack configuration file. Since we're using a WASM file in our project, but not directly reading from it, we need to tell webpack that. Update your `webpack.config.js` file to this:

```javascript
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([{ from: 'index.html', to: 'index.html' }])
  ],
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'javascript/auto',
        loader: 'file-loader',
        options: {
          name: '[name].[ext]',
        },
      },
    ],
  },
  experiments: {
    asyncWebAssembly: true,
  },
};
```

You should now be able to run

> ```shell
> $ npm install
> $ npm start
> ```

And run the project! Feel free to copy paste values from ezkl over to your frontend and see if your proof verifies. 

Thank you for following along and happy hacking!