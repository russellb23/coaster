#![allow(missing_docs)]
use super::api::types as cl;
use super::api::{API, Error};
use memory::*;

use std::ptr;

/// Holds a Cuda memory id and manages its deallocation
#[derive(Debug)]
pub struct Memory {
    /// The underlying memory id>
    memory: cl::memory_id,
    memory_flags: MemoryFlags,

    /// Pointer to host memory that is used for pinned host memory.
    host_ptr: *mut u8,
}

impl Drop for Memory {
    fn drop(&mut self) {
        API::release_memory(self);
        if self.memory_flags.contains(MEM_USE_HOST_PTR) {
            unsafe {
                Box::from_raw(self.host_ptr);
            }
        }
    }
}

bitflags! {
    flags MemoryFlags: cl::bitfield {
        const MEM_READ_WRITE       = 1 << 0,
        const MEM_WRITE_ONLY       = 1 << 1,
        const MEM_READ_ONLY        = 1 << 2,
        const MEM_USE_HOST_PTR     = 1 << 3,
        const MEM_ALLOC_HOST_PTR   = 1 << 4,
        const MEM_COPY_HOST_PTR    = 1 << 5,
    }
}

impl Default for MemoryFlags {
    fn default() -> MemoryFlags {
        MEM_READ_WRITE
    }
}

#[allow(unused_mut)]
impl Memory {
    pub fn new(context: cl::context_id, size: usize) -> Result<Memory, Error> {
        // Memory::from_c(API::create_buffer(...))
        /*
        Memory::<T>::ffi_create_buffer(context,
                                       MEM_READ_WRITE,
                                       size,
                                       ptr::null_mut())
        */
        unimplemented!();
    }

    pub fn from_box(context: cl::context_id, x: Box<u8>) -> Result<Memory, Error> {
        /*Memory::<T>::ffi_create_buffer(context,
                                       MEM_USE_HOST_PTR,
                                       1,
                                       Box::into_raw(x))*/
        unimplemented!();
    }

    pub fn id_c(&self) -> cl::memory_id {
        self.memory
    }

    pub fn from_c(id: cl::memory_id) -> Memory {
        Memory {
            memory: id,
            memory_flags: MemoryFlags::default(),
            host_ptr: ptr::null_mut(),
        }
    }
}

impl IMemory for Memory {}