#![allow(unused)]

use std::borrow::Borrow;

pub const DCD_DATA: &'static [u8] = include_bytes!("dcd.bin");

#[derive(Debug, Clone, Copy)]
pub enum BootDevice {
    SD = 0x400,
    RAM = 0x1000,
}

pub trait AsU8Slice {
    fn as_u8_slice(&self) -> &[u8]
    where
        Self: Sized,
    {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

#[repr(C, packed)]
pub struct ImageVectorTable {
    header: u8,
    length: u16,
    version: u8,
    entry: u32,
    reserved1: u32,
    dcd_address: u32,
    boot_data_address: u32,
    self_address: u32,
    csf_address: u32,
    reserved2: u32,
}

impl ImageVectorTable {
    pub fn new(entry: u32) -> Self {
        ImageVectorTable {
            header: 0xD1_u8,
            length: 0x20_u16.to_be(),
            version: 0x40_u8,
            entry,
            reserved1: 0x0000_0000_u32,
            dcd_address: entry - 0xC00 + 0x2C,
            boot_data_address: entry - 0xC00 + 0x20,
            self_address: entry - 0xC00,
            csf_address: 0x0000_0000_u32,
            reserved2: 0x0000_0000_u32,
        }
    }
}

impl AsU8Slice for ImageVectorTable {}

#[repr(C, packed)]
pub struct BootData {
    start: u32,
    length: u32,
    plugin: u32,
}

impl BootData {
    pub fn new(entry_point: u32, application_size: u32, boot_device: BootDevice) -> Self {
        BootData {
            start: entry_point - boot_device as u32 - 0xC00,
            length: application_size + (boot_device as u32) + 0xC00,
            plugin: 0x0000_0000_u32,
        }
    }
}

impl AsU8Slice for BootData {}
