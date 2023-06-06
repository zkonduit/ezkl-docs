---
icon: cpu
order: 4
---

The cli can also be compiled for the `wasm32-wasi` target (browser bindings with `wasm32-unknown-unknown` under `Tutorials/WASMTutorial`). To do so first ensure that [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) is installed.


You can then run:
```bash
rustup target add wasm32-wasi
```

After adding the wasm32-wasi target, you can use `wasm-pack` to build ezkl for the wasm32-wasi target. The `-Z` flag helps us build unstable features.

```bash
wasm-pack build --bin ezkl --target wasm32-wasi -Z build-std="panic_abort,std"
```

>Note: On Mac you may need to install llvm and clang using homebrew then explicitly set the `CC` and `AR` environment variables. For instance: `AR=/opt/homebrew/opt/llvm/bin/llvm-ar CC=/opt/homebrew/opt/llvm/bin/clang wasm-pack build --bin ezkl --target wasm32-wasi -Z build-std="panic_abort,std"`. You can learn more about how to install these in the WASM tutorial.

You can then run the compiled `.wasm` file as you would the normal cli detailed above (just not the `EVM` related commands), by using [wasmtime](https://docs.wasmtime.dev/cli-install.html). This command runs `ezkl help` on wasmtime.

```bash
wasmtime './target/wasm32-wasi/release/ezkl.wasm' -- --help
```
----------------------