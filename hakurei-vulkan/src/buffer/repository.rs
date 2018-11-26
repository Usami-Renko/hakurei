
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::target::HaBuffer;
use buffer::allocator::BufferAllocateInfos;

use memory::HaMemoryType;
use memory::instance::HaBufferMemory;
use memory::transfer::{ BufferDataUploader, BufferDataUpdater };
use memory::{ AllocatorError, MemoryError };

use types::vkbytes;
use std::marker::PhantomData;

#[derive(Default)]
pub struct HaBufferRepository<M> {

    phantom_type: PhantomData<M>,

    device  : Option<HaDevice>,
    physical: Option<HaPhyDevice>,
    buffers : Vec<HaBuffer>,
    memory  : Option<HaBufferMemory>,

    /// The offset of each buffer in memory.
    offsets: Vec<vkbytes>,

    allocate_infos: Option<BufferAllocateInfos>,
}

impl<M> HaBufferRepository<M> {

    pub(crate) fn store(phantom_type: PhantomData<M>, device: HaDevice, physical: HaPhyDevice, buffers: Vec<HaBuffer>, memory: HaBufferMemory, allocate_infos: BufferAllocateInfos) -> HaBufferRepository<M> {

        use utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&allocate_infos.spaces);

        HaBufferRepository {
            phantom_type,

            device  : Some(device),
            physical: Some(physical),
            memory  : Some(memory),

            buffers, offsets,
            allocate_infos: Some(allocate_infos),
        }
    }

    pub fn data_uploader(&mut self) -> Result<BufferDataUploader, AllocatorError> {

        if let Some(ref mut memory) = self.memory {

            BufferDataUploader::new(
                &self.physical.as_ref().unwrap(),
                &self.device.as_ref().unwrap(),
                memory,
                &self.allocate_infos)
        } else {
            Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }
    }

    pub fn data_updater(&mut self) -> Result<BufferDataUpdater, AllocatorError> {

        if let Some(ref mut memory) = self.memory {

            let updater = match memory.memory_type() {
                | HaMemoryType::HostMemory
                | HaMemoryType::StagingMemory => {
                    BufferDataUpdater::new(
                        &self.device.as_ref().unwrap(),
                        memory,
                        &self.offsets)
                },
                | HaMemoryType::CachedMemory
                | HaMemoryType::DeviceMemory => {
                    return Err(AllocatorError::Memory(MemoryError::MemoryUnableToUpdate))
                },
            };

            Ok(updater)
        } else {
            Err(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))
        }
    }

    pub fn cleanup(&mut self) {

        self.buffers.iter().for_each(|buffer|
            buffer.cleanup(&self.device.as_ref().unwrap()));

        if let Some(ref mut memory) = self.memory {
            memory.cleanup(&self.device.as_ref().unwrap());
        }

        self.buffers.clear();
        self.offsets.clear();
    }
}
