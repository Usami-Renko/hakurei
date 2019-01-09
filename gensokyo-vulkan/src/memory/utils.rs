
use ash::vk;

use crate::memory::traits::MemoryMappable;

use crate::types::{ vkptr, vkbytes };

#[derive(Debug, Clone, Copy)]
pub struct MemoryRange {

    pub offset: vkbytes,
    pub size  : vkbytes,
}

#[derive(Debug, Clone)]
pub struct MemoryMapStatus {

    /// The begining data ptr of the whole memory.
    data_ptr: Option<vkptr>,
}

impl MemoryMapStatus {

    pub fn from_unmap() -> MemoryMapStatus {

        MemoryMapStatus {
            data_ptr: None,
        }
    }

    pub unsafe fn data_ptr(&self, offset: vkbytes) -> Option<vkptr> {

        self.data_ptr.and_then(|ptr| {
            Some(ptr.offset(offset as isize))
        })
    }

    pub fn set_map(&mut self, ptr: vkptr) {

        self.data_ptr = Some(ptr);
    }

    pub fn invaild_map(&mut self) {

        self.data_ptr = None;
    }

    pub fn is_mapping(&self) -> bool {

        self.data_ptr.is_some()
    }
}

pub struct MemoryMapAlias {

    pub handle: vk::DeviceMemory,
    pub status: MemoryMapStatus,
    pub is_coherent: bool,
}

impl MemoryMappable for MemoryMapAlias {

    fn map_handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.status
    }
}

pub struct MemoryWritePtr {

    ptr: vkptr,
    size: vkbytes,
}

impl MemoryWritePtr {

    pub fn new(ptr: vkptr, size: vkbytes) -> MemoryWritePtr {
        MemoryWritePtr { ptr, size }
    }

    pub fn write_data<D: Copy>(&self, data: &[D]) {

        use std::mem;

        let mut vert_algn = unsafe {
            ash::util::Align::new(
                self.ptr,
                mem::align_of::<D>() as vkbytes,
                self.size,
            )
        };

        vert_algn.copy_from_slice(data);
    }
}