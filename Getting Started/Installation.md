---
icon: tools
order: 96
---

+++ Local CLI
1. Install the binary with the following bash script.
```bash
curl https://raw.githubusercontent.com/zkonduit/ezkl/main/install_ezkl_cli.sh | bash
```

2. Alternatively, build from source:
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

1. Install `archon` with the following script
```bash
curl https://download.ezkl.xyz/download_archon.sh | bash
```

2. If your system settings block the installed binary you will need to allow your system to run the binary.

3. Set the server environment variable:
```bash
export ARCHON_SERVER_URL="https://archon-v0.ezkl.xyz"
```
4. Test the connection:
```bash
archon ping
```
+++
## Additional Notes
- While there is some support for Windows with the original `ezkl` repository, unfortunately Lilith does not work on Windows systems.
- EZKL uses your system's `solc` Solidity compiler. You may need to adjust it using `svm-rs` or `solc-select`.
- For rendering model circuits with EZKL, compile with the `render` feature and install required libraries (e.g., `libexpat1-dev` and `libfreetype6-dev` on Debian systems).
- For EZKL Rust documentation, use `cargo doc --open`.