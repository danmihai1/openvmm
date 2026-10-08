// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Rust binadings to the `vmservice.proto` TTRPC API

#![expect(missing_docs)]
#![forbid(unsafe_code)]
#![expect(clippy::enum_variant_names, clippy::large_enum_variant)]
#![expect(clippy::allow_attributes)]

// Crates used by generated code. Reference them explicitly to ensure that
// automated tools do not remove them.
use mesh_rpc as _;
use prost as _;

/// Compatibility version for the VM service API.
pub const API_VERSION: u32 = 1;

/// Source revision supplied by the build environment.
pub const BUILD_REVISION: &str = match option_env!("OPENVMM_BUILD_REVISION") {
    Some(revision) => revision,
    None => "unknown",
};

/// FNV-1a fingerprint of the exact VM service schema used by this crate.
pub const SCHEMA_FINGERPRINT: u64 = schema_fingerprint(include_bytes!("vmservice.proto"));

const fn schema_fingerprint(schema: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325;
    let mut index = 0;
    while index < schema.len() {
        hash ^= schema[index] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        index += 1;
    }
    hash
}

include!(concat!(env!("OUT_DIR"), "/vmservice.rs"));
