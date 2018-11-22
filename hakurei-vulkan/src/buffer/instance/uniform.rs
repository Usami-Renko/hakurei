
use ash::vk;

use buffer::target::BufferDescInfo;
use buffer::entity::BufferBlock;
use buffer::instance::enums::BufferInstanceType;
use buffer::traits::{ BufferInstance, BufferBlockInfo };
use buffer::traits::{ BufferCopiable, BufferCopyInfo };
use descriptor::DescriptorBufferBindableTarget;
use descriptor::{ DescriptorBindingContent, DescriptorBufferBindingInfo };
use descriptor::{ HaDescriptorType, BufferDescriptorType };

use types::{ vkuint, vkbytes };

#[derive(Debug, Clone)]
pub struct UniformBlockInfo {

    info: BufferDescInfo,
    binding: DescriptorBindingContent,
    element_size: vkbytes,
}

impl UniformBlockInfo {

    pub fn new(binding: vkuint, count: vkuint, element_size: vkbytes) -> UniformBlockInfo {

        let estimate_size = (count * count) as vkbytes;

        UniformBlockInfo {
            info: BufferDescInfo::new(estimate_size, vk::BufferUsageFlags::UNIFORM_BUFFER),
            binding: DescriptorBindingContent {
                binding, count,
                descriptor_type: HaDescriptorType::Buffer(BufferDescriptorType::UniformBuffer),
            },
            element_size,
        }
    }
}

impl BufferBlockInfo for UniformBlockInfo {
    const INSTANCE_TYPE: BufferInstanceType = BufferInstanceType::UniformBuffer;

    fn as_desc_ref(&self) -> &BufferDescInfo {
        &self.info
    }

    fn into_desc(self) -> BufferDescInfo {
        self.info
    }
}

pub struct HaUniformBlock {

    binding: DescriptorBindingContent,

    block: BufferBlock,
    repository_index: usize,
    element_size: vkbytes,
}

impl HaUniformBlock {

    pub(super) fn new(info: &UniformBlockInfo, block: BufferBlock, repository_index: usize) -> HaUniformBlock {

        HaUniformBlock {
            binding: info.binding.clone(),
            element_size: info.element_size,
            block,
            repository_index,
        }
    }
}

impl DescriptorBufferBindableTarget for HaUniformBlock {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo {

        DescriptorBufferBindingInfo {
            content: self.binding.clone(),
            element_indices: sub_block_indices.unwrap_or(vec![0]),
            element_size: self.element_size,
            buffer: &self.block,
        }
    }
}

impl BufferInstance for HaUniformBlock {

    fn typ(&self) -> BufferInstanceType {
        BufferInstanceType::UniformBuffer
    }

    fn as_block_ref(&self) -> &BufferBlock {
        &self.block
    }
}

impl BufferCopiable for HaUniformBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
