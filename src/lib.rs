//! Rusty wrapper for the Unified Extensible Firmware Interface.
//!
//! # Crate organisation
//!
//! The top-level module contains some of the most used types,
//! such as the result and error types, or other common data structures
//! such as GUIDs and handles.
//!
//! ## Tables and protocols
//!
//! The `table` module contains definitions of the UEFI tables,
//! which are structures containing some basic functions and references to other tables.
//! Most importantly, the boot services table also provides a way to obtain **protocol** handles.
//!
//! The `proto` module contains the standard UEFI protocols, which are normally provided
//! by the various UEFI drivers and firmware layers.
//!
//! ## Adapting to local conditions
//!
//! Unlike system tables, which are present on *all* UEFI implementations,
//! protocols *may* or *may not* be present on a certain system.
//!
//! For example, a PC with no network card might not contain a network driver,
//! therefore all the network protocols will be unavailable.

#![cfg_attr(
    feature = "exts",
    feature(allocator_api, alloc_layout_extra, vec_spare_capacity)
)]
#![feature(auto_traits)]
#![feature(control_flow_enum)]
#![feature(try_trait_v2)]
#![feature(abi_efiapi)]
#![feature(negative_impls)]
#![no_std]
// Enable some additional warnings and lints.
#![warn(missing_docs, unused)]
#![deny(clippy::all)]

// `uefi-exts` requires access to memory allocation APIs.
#[cfg(feature = "exts")]
extern crate alloc as alloc_api;

// allow referring to self as ::uefi for macros to work universally (from this crate and from others)
// see https://github.com/rust-lang/rust/issues/54647
extern crate self as uefi;

#[macro_use]
pub mod data_types;
#[cfg(feature = "exts")]
pub use self::data_types::CString16;
pub use self::data_types::{unsafe_guid, Identify};
pub use self::data_types::{CStr16, CStr8, Char16, Char8, Event, Guid, Handle};

mod result;
pub use self::result::{Completion, Result, ResultExt, Status};

pub mod table;

pub mod proto;

pub mod prelude;

#[cfg(feature = "alloc")]
pub mod alloc;

#[cfg(feature = "exts")]
pub mod exts;

#[cfg(feature = "logger")]
pub mod logger;
