# uefi-rs

[![Crates.io](https://img.shields.io/crates/v/uefi)](https://crates.io/crates/uefi)
[![Docs.rs](https://docs.rs/uefi/badge.svg)](https://docs.rs/uefi)
![Stars](https://img.shields.io/github/stars/rust-osdev/uefi-rs)
![License](https://img.shields.io/github/license/rust-osdev/uefi-rs)
![Build status](https://github.com/rust-osdev/uefi-rs/workflows/Rust/badge.svg)

## Description

[UEFI] is the successor to the BIOS. It provides an early boot environment for
OS loaders, hypervisors and other low-level applications. While it started out
as x86-specific, it has been adopted on other platforms, such as ARM.

This crate makes it easy to both:
  - Write UEFI applications in Rust (for `i686`, `x86_64`, or `aarch64`)
  - Call UEFI functions from an OS (usually built with a [custom target][rustc-custom])

The objective is to provide **safe** and **performant** wrappers for UEFI interfaces,
and allow developers to write idiomatic Rust code.

Check out [the UEFI application template](template) for a quick start.

**Note**: this crate currently has only been tested with **64-bit** UEFI on x86/ARM.

[UEFI]: https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface
[rustc-custom]: https://doc.rust-lang.org/rustc/targets/custom.html

![uefi-rs running in QEMU](https://imgur.com/SFPSVuO.png)

## Project structure

This project contains multiple sub-crates:

- `uefi` (top directory): defines the standard UEFI tables / interfaces.
  The objective is to stay unopionated and safely wrap most interfaces.

  Optional features:
  - `alloc`: implements a global allocator using UEFI functions.
    - This allows you to allocate objects on the heap.
    - There's no guarantee of the efficiency of UEFI's allocator.
  - `logger`: logging implementation for the standard [log] crate.
    - Prints output to console.
    - No buffering is done: this is not a high-performance logger.
  - `exts`: extensions providing utility functions for common patterns.
    - Requires the `alloc` crate (either enable the `alloc` optional feature or your own custom allocator).

- `uefi-macros`: procedural macros that are used to derive some traits in `uefi`.

- `uefi-services`: provides a panic handler, and initializes the `alloc` / `logger` features.

- `uefi-test-runner`: a UEFI application that runs unit / integration tests.

[log]: https://github.com/rust-lang-nursery/log

## Documentation

The docs for the latest published crate version can be found at
[docs.rs/uefi/](https://docs.rs/uefi/)

This crate's documentation is fairly minimal, and you are encouraged to refer to
the [UEFI specification][spec] for detailed information.

[spec]: http://www.uefi.org/specifications

## Building and testing uefi-rs

Install the `nightly` version of Rust and the `rust-src` component:
```
rustup toolchain install nightly
rustup component add --toolchain nightly rust-src
```

Use the `./build.py` script to build and test the crate.

Available commands:
- `build`: build `uefi-test-runner`
- `clippy`: run clippy on the whole workspace
- `doc`: build the docs for the library packages
- `run`: build `uefi-test-runner` and run it in QEMU
- `test`: run unit tests and doctests on the host

Available options:
- `--target {x86_64,aarch64}`: choose which architecture to build/run the tests
- `--verbose`: enables verbose mode, prints commands before running them
- `--headless`: enables headless mode, which runs QEMU without a GUI
- `--release`: builds the code with optimizations enabled
- `--disable-kvm`: disable [KVM](https://www.linux-kvm.org/page/Main_Page) hardware acceleration
  when running the tests in QEMU (especially useful if you want to run the tests under
  [WSL](https://docs.microsoft.com/en-us/windows/wsl) on Windows.

The `uefi-test-runner` directory contains a sample UEFI app which exercises
most of the library's functionality.

Check out the testing project's [`README.md`](uefi-test-runner/README.md) for instructions on how to run the tests.

## Building UEFI programs

For instructions on how to create your own UEFI apps, see the [BUILDING.md](BUILDING.md) file.

## Contributing

We welcome issues and pull requests! For instructions on how to set up a development
environment and how to add new protocols, check out [CONTRIBUTING.md](CONTRIBUTING.md).

## License

The code in this repository is licensed under the Mozilla Public License 2.
This license allows you to use the crate in proprietary programs, but any modifications to the files must be open-sourced.

The full text of the license is available in the [license file](LICENSE).
