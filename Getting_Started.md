---
icon: gear
order: 9
---

### building the project 🔨
Note that the library requires a nightly version of the rust toolchain. You can change the default toolchain by running:

```bash
rustup override set nightly
```

After which you may build the library

```bash
cargo build --release
```

A folder `./target/release` will be generated. Add this folder to your PATH environment variable to call `ezkl` from the CLI.

```bash
# For UNIX like systems
# in .bashrc, .bash_profile, or some other path file
export PATH="<replace with where you cloned the repo>/ezkl/target/release:$PATH"
```

Restart your shell or reload your shell settings

```bash
# example, replace .bash_profile with the file you use to configure your shell
source ~/.bash_profile
```

You will need a functioning installation of `solc` in order to run `ezkl` properly.
[solc-select](https://github.com/crytic/solc-select) is recommended.
Follow the instructions on [solc-select](https://github.com/crytic/solc-select) to activate `solc` in your environment.

### docs 📖

Use `cargo doc --open` to compile and open the docs in your default browser.