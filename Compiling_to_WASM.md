---
icon: cpu
order: 4
---

The cli can also be compiled to for `wasm32-wasi` target (browser bindings with `wasm32-unknown-unknown` coming soon). To do so first ensure that [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) is installed.


You can then run:
```bash
rustup target add wasm32-wasi

wasm-pack build --bin ezkl --target wasm32-wasi
```
>Note: On Mac you may need to install llvm and clang using homebrew then explicitly set the `CC` and `AR` environment variables. For instance: `AR=/opt/homebrew/opt/llvm/bin/llvm-ar CC=/opt/homebrew/opt/llvm/bin/clang wasm-pack build --bin ezkl --target wasm32-wasi`

You can then run the compiled `.wasm` file as you would the normal cli detailed above (just not the `EVM` related commands), by using [wasmtime](https://docs.wasmtime.dev/cli-install.html).

```bash
wasmtime './target/wasm32-wasi/release/ezkl.wasm' -- --help
```
----------------------