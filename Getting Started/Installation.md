---
icon: tools
order: 96
---

+++ Local CLI
1. Download the release binary from GitHub, or
2. Build from source:
```bash
git clone git@github.com:zkonduit/ezkl.git
cd ezkl
cargo install --force --path .
```
For detailed options, use `ezkl --help` or `ezkl <command> --help`.

+++ Python
Install EZKL using pip:
```bash
pip install ezkl
```
You may also need to install additional dependencies:
- `onnx` for exporting models
- PyTorch or TensorFlow for creating models

To use EZKL in your Python code:
```bash
import ezkl
```
+++ JavaScript
Install EZKL using npm:
```bash
npm install @ezkljs/engine
```
To use EZKL in your JavaScript code:
```bash
import { Engine } from '@ezkljs/engine';
```
+++ Remote CLI

1. Download and extract the appropriate binary:
- Mac (Apple Silicon): `build-artifacts.lilith-macos-aarch64.tar.gz`
- Mac (Intel): `build-artifacts.lilith-macos-aarch64.tar.gz`
- Linux (GNU): `build-artifacts.lilith-linux-gnu.tar.gz`

2. Place the `lilith` executable in your system path:
* Mac: `~/.local/bin/`
* Linux: `/usr/local/bin/`

3. Set the server environment variable:
```bash
export LILITH_SERVER_URL="http://15.204.235.127:2004"
```
4. Test the connection:
```bash
lilith ping
```
+++
## Additional Notes
- While there is some support for Windows with the original `ezkl` repository, unfortunately Lilith does not work on Windows systems.
- EZKL uses your system's `solc` Solidity compiler. You may need to adjust it using `svm-rs` or `solc-select`.
- For rendering model circuits with EZKL, compile with the `render` feature and install required libraries (e.g., `libexpat1-dev` and `libfreetype6-dev` on Debian systems).
- For EZKL Rust documentation, use `cargo doc --open`.