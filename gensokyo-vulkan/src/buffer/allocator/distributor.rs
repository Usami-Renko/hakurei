
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::{ GsBuffer, BufferBlock };
use crate::buffer::allocator::BufferBlockIndex;
use crate::buffer::allocator::memory::BufferAllocateInfos;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;
use crate::buffer::instance::{ GsVertexBlock, GsIndexBlock, GsUniformBlock, GsImgsrcBlock };
use crate::buffer::repository::GsBufferRepository;
use crate::memory::instance::GsBufferMemory;
use crate::memory::AllocatorError;

use crate::types::vkbytes;

use std::marker::PhantomData;

pub struct GsBufferDistributor<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device   : GsDevice,
    physical : GsPhyDevice,
    memory   : GsBufferMemory,

    buffers : Vec<GsBuffer>,
    spaces  : Vec<vkbytes>,
    offsets : Vec<vkbytes>,

    allot_infos: BufferAllocateInfos,
}

impl<M> GsBufferDistributor<M> where M: BufferMemoryTypeAbs {

    pub(super) fn new(phantom_type: PhantomData<M>, device: GsDevice, physical: GsPhyDevice, memory: GsBufferMemory, buffers: Vec<GsBuffer>, spaces: Vec<vkbytes>, allot_infos: BufferAllocateInfos) -> GsBufferDistributor<M> {

        use crate::utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&spaces);

        GsBufferDistributor {
            phantom_type, device, physical, memory, buffers, spaces, offsets, allot_infos,
        }
    }

    pub fn acquire_vertex(&self, index: BufferBlockIndex) -> GsVertexBlock {

        GsVertexBlock::new(self.gen_buffer_item(&index), index)
    }

    pub fn acquire_index(&self, index: BufferBlockIndex) -> GsIndexBlock {

        GsIndexBlock::new(self.gen_buffer_item(&index), index)
    }

    pub fn acquire_uniform(&self, index: BufferBlockIndex) ->  Result<GsUniformBlock, AllocatorError> {

        let block = GsUniformBlock::new(self.gen_buffer_item(&index), index)?;
        Ok(block)
    }

    pub fn acquire_imgsrc(&self, index: BufferBlockIndex) -> GsImgsrcBlock {

        GsImgsrcBlock::new(self.gen_buffer_item(&index), index)
    }

    pub fn into_repository(self) -> GsBufferRepository<M> {

        GsBufferRepository::store(self.phantom_type, self.device, self.physical, self.buffers, self.memory, self.allot_infos)
    }

    fn gen_buffer_item(&self, index: &BufferBlockIndex) -> BufferBlock {

        BufferBlock::new(
            &self.buffers[index.value],
            self.spaces[index.value],
            self.offsets[index.value],
        )
    }
}
