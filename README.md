# Rust Introduction

To install Rust: https://www.rust-lang.org/tools/install

To run code:
```bash
cargo run --bin file_name_without_extension 
```

To add new files, you must declare them in the ```Cargo.toml``` file under a new ```[[bin]]``` with the ```name``` property set to the name of the Rust file without the extension 

```toml
[[bin]]
name = "intro"

[[bin]]
name = "pres"
```
To start a new Rust project: ```cargo init```
