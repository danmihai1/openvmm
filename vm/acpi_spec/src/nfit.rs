// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! NVDIMM Firmware Interface Table (NFIT) definitions.

use crate::Table;
use crate::packed_nums::u16_ne;
use crate::packed_nums::u32_ne;
use crate::packed_nums::u64_ne;
use core::mem::size_of;
use static_assertions::const_assert_eq;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;
use zerocopy::Unaligned;

/// Byte-addressable persistent memory region GUID.
pub const SPA_RANGE_PERSISTENT_MEMORY_GUID: [u8; 16] = [
    0x79, 0xd3, 0xf0, 0x66, 0xf3, 0xb4, 0x74, 0x40, 0xac, 0x43, 0x0d, 0x33, 0x18, 0xb7, 0x8c, 0xdb,
];

/// The NVDIMM cannot accept persistent writes.
pub const MEMORY_DEVICE_STATE_NOT_ARMED: u16 = 1 << 3;

/// UEFI write-back memory attribute.
pub const EFI_MEMORY_WB: u64 = 0x8;

/// UEFI non-volatile memory attribute.
pub const EFI_MEMORY_NV: u64 = 0x8000;

/// NFIT table body.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, IntoBytes, Immutable, KnownLayout, FromBytes, Unaligned)]
pub struct Nfit {
    pub reserved: u32_ne,
}

impl Table for Nfit {
    const SIGNATURE: [u8; 4] = *b"NFIT";
}

const_assert_eq!(size_of::<Nfit>(), 4);

/// Common NFIT structure header.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, IntoBytes, Immutable, KnownLayout, FromBytes, Unaligned)]
pub struct StructureHeader {
    pub structure_type: u16_ne,
    pub length: u16_ne,
}

impl StructureHeader {
    pub fn new<T>(structure_type: u16) -> Self {
        Self {
            structure_type: structure_type.into(),
            length: (size_of::<T>() as u16).into(),
        }
    }
}

const_assert_eq!(size_of::<StructureHeader>(), 4);

/// System Physical Address Range Structure (type 0).
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, IntoBytes, Immutable, KnownLayout, FromBytes, Unaligned)]
pub struct SpaRange {
    pub header: StructureHeader,
    pub spa_range_index: u16_ne,
    pub flags: u16_ne,
    pub reserved: u32_ne,
    pub proximity_domain: u32_ne,
    pub address_range_type_guid: [u8; 16],
    pub spa_base: u64_ne,
    pub spa_length: u64_ne,
    pub memory_mapping_attributes: u64_ne,
}

const_assert_eq!(size_of::<SpaRange>(), 56);

/// Memory Device to System Physical Address Range Mapping Structure (type 1).
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, IntoBytes, Immutable, KnownLayout, FromBytes, Unaligned)]
pub struct MemoryDeviceMapping {
    pub header: StructureHeader,
    pub device_handle: u32_ne,
    pub physical_id: u16_ne,
    pub region_id: u16_ne,
    pub spa_range_index: u16_ne,
    pub control_region_index: u16_ne,
    pub region_size: u64_ne,
    pub region_offset: u64_ne,
    pub address_region_base: u64_ne,
    pub interleave_index: u16_ne,
    pub interleave_ways: u16_ne,
    pub state_flags: u16_ne,
    pub reserved: u16_ne,
}

const_assert_eq!(size_of::<MemoryDeviceMapping>(), 48);

/// NVDIMM Control Region Structure (type 4).
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, IntoBytes, Immutable, KnownLayout, FromBytes, Unaligned)]
pub struct ControlRegion {
    pub header: StructureHeader,
    pub control_region_index: u16_ne,
    pub vendor_id: u16_ne,
    pub device_id: u16_ne,
    pub revision_id: u16_ne,
    pub subsystem_vendor_id: u16_ne,
    pub subsystem_device_id: u16_ne,
    pub subsystem_revision_id: u16_ne,
    pub valid_fields: u8,
    pub manufacturing_location: u8,
    pub manufacturing_date: u16_ne,
    pub reserved: [u8; 2],
    pub serial_number: u32_ne,
    pub region_format_interface_code: u16_ne,
    pub block_control_window_count: u16_ne,
    pub block_control_window_size: u64_ne,
    pub command_register_offset: u64_ne,
    pub command_register_size: u64_ne,
    pub status_register_offset: u64_ne,
    pub status_register_size: u64_ne,
    pub flags: u16_ne,
    pub reserved2: [u8; 6],
}

const_assert_eq!(size_of::<ControlRegion>(), 80);
