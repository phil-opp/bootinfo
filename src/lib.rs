#![deny(improper_ctypes)]
#![no_std]

use core::ops::Deref;
use core::slice;
pub use memory_map::*;

const VERSION: u64 = 5;

mod memory_map;

#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    pub version: u64,
    pub p4_table_addr: u64,
    pub memory_map: MemoryMap,
    pub package: Package,
}

#[derive(Debug)]
#[repr(C)]
pub struct Package {
    ptr: *const u8,
    len: u64,
}

impl Deref for Package {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len as usize) }
    }
}

impl BootInfo {
    pub fn new(p4_table_addr: u64, memory_map: MemoryMap, package: &'static [u8]) -> Self {
        BootInfo {
            version: VERSION,
            p4_table_addr,
            memory_map,
            package: Package {
                ptr: package.as_ptr(),
                len: package.len() as u64,
            },
        }
    }

    pub fn check_version(&self) -> Result<(), ()> {
        if self.version == VERSION {
            Ok(())
        } else {
            Err(())
        }
    }
}

extern "C" {
    fn _improper_ctypes_check(_boot_info: BootInfo);
}
