
use crate::core::GsDevice;

use crate::memory::GsMemoryAbstract;
use crate::memory::transfer::MemoryDataDelegate;
use crate::error::VkResult;

use crate::buffer::allocator::BufferAllocateInfos;

pub type GsBufferMemory = Box<dyn GsBufferMemoryAbs>;
pub type GsImageMemory  = Box<dyn GsImageMemoryAbs>;

pub trait GsBufferMemoryAbs: GsMemoryAbstract {

    fn to_upload_agency(&self, device: &GsDevice, allot_infos: &BufferAllocateInfos) -> VkResult<Box<dyn MemoryDataDelegate>>;
    fn to_update_agency(&self) -> VkResult<Box<dyn MemoryDataDelegate>>;
}

pub trait GsImageMemoryAbs: GsMemoryAbstract {}
