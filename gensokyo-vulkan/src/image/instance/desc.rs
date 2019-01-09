
use ash::vk;

use crate::types::vkDim3D;

#[derive(Debug, Default)]
pub struct ImageInstanceInfoDesc {

    pub current_layout: vk::ImageLayout,
    pub dimension: vkDim3D,
    pub subrange: vk::ImageSubresourceRange,
}