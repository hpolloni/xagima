## XagimaOS
A toy OS

### Building (MacOS)
```sh
rustup component add rust-src --toolchain nightly-x86_64-apple-darwin

cargo build
```

### Running it
You might need to install certain components.
```sh
# Needed by bootloader crate
rustup component add llvm-tools-preview

brew install qemu

cargo run
```