//! # Semver Compatibility
//!
//! cargo-release's versioning tracks compatibility for the binaries, not the API.  We upload to
//! crates.io to distribute the binary.  If using this as a library, be sure to pin the version
//! with a `=` version requirement operator.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub mod error;
pub mod utils;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
