
use ash::vk;

use core::device::HaDevice;

use memory::{ HaMemoryType, MemoryMapable };
use memory::MemoryError;

#[derive(Debug, Clone, Copy)]
pub struct Host;
#[derive(Debug, Clone, Copy)]
pub struct Cached;
#[derive(Debug, Clone, Copy)]
pub struct Device;
#[derive(Debug, Clone, Copy)]
pub struct Staging;

pub trait BufferMemoryTypeAbs {
    const MEMORY_TYPE: HaMemoryType;

    fn memory_type(&self) -> HaMemoryType {
        Self::MEMORY_TYPE
    }

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags;

    fn map_memory_if_need(&self, _device: &HaDevice, _mapable_memory: &mut MemoryMapable) -> Result<(), MemoryError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferStorageType {}

impl BufferStorageType {

    pub const HOST   : Host    = Host;
    pub const DEVICE : Device  = Device;
    pub const CACHED : Cached  = Cached;
    pub const STAGING: Staging = Staging;
}

impl BufferMemoryTypeAbs for Host {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::HostMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin
    }

    fn map_memory_if_need(&self, device: &HaDevice, mapable_memory: &mut MemoryMapable) -> Result<(), MemoryError> {
        mapable_memory.map_range(device, None)
    }
}

impl BufferMemoryTypeAbs for Cached {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::CachedMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin | vk::BufferUsageFlags::TRANSFER_DST
    }
}

impl BufferMemoryTypeAbs for Device {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::DeviceMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin | vk::BufferUsageFlags::TRANSFER_DST
    }
}

impl BufferMemoryTypeAbs for Staging {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::StagingMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin| vk::BufferUsageFlags::TRANSFER_SRC
    }
}
