# How to contribute to uefi-rs

Pull requests, issues and suggestions are welcome!

The UEFI spec is huge, so there might be some omissions or some missing features.
You should follow the existing project structure when adding new items.

See the top-level [README](../README.md) for details of using `./build.py` to
build and test the project.

Make some changes in your favourite editor / IDE:
I use [VS Code][code] with the [RLS][rls] extension.

Test your changes:

```shell
./build.py run
```

The line above will open a QEMU window where the test harness will run some tests.

Any contributions are also expected to pass [Clippy][clippy]'s static analysis,
which you can run as follows:

```shell
./build.py clippy
```

[clippy]: https://github.com/rust-lang-nursery/rust-clippy
[code]: https://code.visualstudio.com/
[rls]: https://github.com/rust-lang-nursery/rls-vscode

## Style guide

This repository follows Rust's [standard style][style], the same one imposed by `rustfmt`.

You can apply the standard style to the whole package by running `cargo fmt --all`.

[style]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md

## UEFI pitfalls

Interfacing with a foreign and unsafe API is a difficult exercise in general, and
UEFI is certainly no exception. This section lists some common pain points that
you should keep in mind while working on UEFI interfaces.

### Enums

Rust and C enums differ in many way. One safety-critical difference is that the
Rust compiler assumes that all variants of Rust enums are known at compile-time.
UEFI, on the other hand, features many C enums which can be freely extended by
implementations or future versions of the spec.

These enums must not be interfaced as Rust enums, as that could lead to undefined
behavior. Instead, integer newtypes with associated constants should be used. The
`newtype_enum` macro is provided by this crate to ease this exercise.

### Pointers

Pointer parameters in UEFI APIs come with many safety conditions. Some of these
are usually expected by unsafe Rust code, while others are more specific to the
low-level environment that UEFI operates in:

- Pointers must reference physical memory (no memory-mapped hardware)
- Pointers must be properly aligned for their target type
- Pointers may only be NULL where UEFI explicitly allows for it
- When an UEFI function fails, nothing can be assumed about the state of data
  behind `*mut` pointers.

## Adding new protocols

You should start by [forking this repository][fork] and cloning it.

UEFI protocols are represented in memory as tables of function pointers,
each of which takes the protocol itself as first parameter.

In `uefi-rs`, protocols are simply `struct`s containing `extern "efiapi" fn`s.
It's imperative to add `#[repr(C)]` to ensure the functions are laid out in memory
in the order the UEFI spec requires.

Each protocol also has a Globally Unique Identifier (in the C API, they're usually
found in a `EFI_*_PROTOCOL_GUID` define). In Rust, we store the GUID as an associated
constant, by implementing the unsafe trait `uefi::proto::Identify`. For convenience,
this is done through the `unsafe_guid` macro.

Finally, you should derive the `Protocol` trait. This is a marker trait,
extending `Identify`, which is used as a generic bound in the functions which retrieve
protocol implementations.

An example protocol declaration:

```rust
/// Protocol which does something.
#[repr(C)]
#[unsafe_guid("abcdefgh-1234-5678-9012-123456789abc")]
#[derive(Protocol)]
pub struct NewProtocol {
  some_entry_point: extern "efiapi" fn(
    this: *const NewProtocol,
    some_parameter: SomeType,
    some_other_parameter: SomeOtherType,
  ) -> Status,
  some_other_entry_point: extern "efiapi" fn(
    this: *mut NewProtocol,
    another_parameter: AnotherType,
  ) -> SomeOtherResult,
  // ...
}
```

There should also be an `impl` block providing safe access to the functions:

```rust
impl NewProtocol {
  /// This function does something.
  pub fn do_something(&self, a: SomeType, b: SomeOtherType) -> Result {
    // Call the wrapped function
    let status = unsafe { (self.some_entry_point)(self, a, b) };
    // `Status` provides a helper function for converting to `Result`
    status.into()
  }
}
```

[fork]: https://docs.github.com/en/free-pro-team@latest/github/getting-started-with-github/fork-a-repo

## Publishing new versions of the crates

Maintainers of this repository might also be interested in [the guidelines](PUBLISHING.md)
for publishing new versions of the uefi-rs crates.
