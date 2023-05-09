---
icon: gear
order: 8
---

### building the project 🔨
Note that the library requires a nightly version of the rust toolchain. You can change the default toolchain by running:
```bash
rustup override set nightly
```

After which you may build the library
```bash
cargo install --force --path
```

If you're optimizing for speed, run:
```bash
cargo build --release
```
---------
### Rust docs 📖

Use `cargo doc --open` to compile and open the Rust documentation for `ezkl` in your default browser. We will also have a live link with our Rust documentation available soon.