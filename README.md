# Rust Bindings for tonlibjson Library

This repository contains the Rust bindings for tonlib library (https://github.com/ton-blockchain/ton), allowing developers to use tonlib functionality in their Rust applications.

## Features
* Uses Cmake to build tonlibjson_static by default.
* Supports shared tonlib. You can build with --features shared-tonlib.

## Usage
This library is used in the tonlib-rs library (https://github.com/ston-fi/tonlib-rs), which provides a higher-level Rust interface to the tonlib functionality.

To use this library in your Rust application, add the following to your Cargo.toml file:

```toml
[dependencies]
tonlib-json = "0.1.0"
```

Then, in your Rust code, you can import the library with:

```rust
use tonlib_json;
```

## Contributing

If you want to contribute to this library, please feel free to open a pull request on GitHub.

## License
This library is licensed under the MIT license. See the LICENSE file for details.