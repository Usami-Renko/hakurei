
use ash::vk;

use crate::core::device::GsDevice;

use crate::descriptor::layout::{ GsDescriptorSetLayout, DescriptorSetLayoutInfo };
use crate::descriptor::entity::DescriptorSetEntity;
use crate::descriptor::binding::DescriptorBindingInfo;
use crate::descriptor::binding::{ DescriptorBufferBindableTarget, DescriptorImageBindableTarget };

use std::slice::Iter;

pub struct GsDescriptorSet {

    pub(crate) handle: vk::DescriptorSet,
    layout: GsDescriptorSetLayout,
}

impl GsDescriptorSet {

    pub(crate) fn new(handle: vk::DescriptorSet, layout: GsDescriptorSetLayout) -> GsDescriptorSet {

        GsDescriptorSet {
            handle, layout,
        }
    }

    pub fn layout(&self) -> GsDescriptorSetLayout {
        self.layout.clone()
    }

    pub fn destroy(&self, device: &GsDevice) {
        self.layout.destroy(device);
    }
}


#[derive(Default)]
pub struct DescriptorSetConfig {

    layout_flags: vk::DescriptorSetLayoutCreateFlags,

    bindings    : Vec<Box<dyn DescriptorBindingInfo>>,
    stage_flags : Vec<vk::ShaderStageFlags>,
}

impl DescriptorSetConfig {

    pub fn init(layout_flags: vk::DescriptorSetLayoutCreateFlags) -> DescriptorSetConfig {
        DescriptorSetConfig {
            layout_flags,
            ..Default::default()
        }
    }

    pub fn add_buffer_binding(&mut self, bind_target: &impl DescriptorBufferBindableTarget, stage: GsDescBindingStage) {

        let binding_info = bind_target.binding_info(None);
        self.add_binding(Box::new(binding_info), stage);
    }

    pub fn add_image_binding(&mut self, bind_target: &impl DescriptorImageBindableTarget, stage: GsDescBindingStage) {

        let binding_info = bind_target.binding_info();
        self.add_binding(Box::new(binding_info), stage);
    }

    fn add_binding(&mut self, binding: Box<dyn DescriptorBindingInfo>, stage: GsDescBindingStage) {

        self.bindings.push(binding);
        self.stage_flags.push(stage.0);
    }

    pub fn to_layout_info(&self) -> DescriptorSetLayoutInfo {

        let mut layout_info = DescriptorSetLayoutInfo::setup(self.layout_flags);
        for (i, info) in self.bindings.iter().enumerate() {
            layout_info.add_binding(info, self.stage_flags[i]);
        }

        layout_info
    }

    pub fn iter_binding(&self) -> Iter<Box<dyn DescriptorBindingInfo>> {
        self.bindings.iter()
    }
}


pub struct DescriptorSet {

    pub(crate) entity: DescriptorSetEntity,
    pub(crate) layout: GsDescriptorSetLayout,

    /// `set_index` is the `set` value used in shader code, like the following example shader snippet:
    ///
    /// layout (set = 1, binding = 0) uniform UniformBlock { mat4 projection; }
    set_index: usize,
}

impl DescriptorSet {

    pub fn new(from: &GsDescriptorSet, config: &DescriptorSetConfig, set_index: usize) -> DescriptorSet {

        let binding_indices = config.iter_binding()
            .map(|b| b.borrow_binding_content().binding)
            .collect();

        DescriptorSet {
            entity: DescriptorSetEntity::new(from, binding_indices),
            layout: from.layout(),
            set_index,
        }
    }

    pub fn set_index(&self) -> usize {
        self.set_index.clone()
    }
}


pub struct GsDescBindingStage(vk::ShaderStageFlags);

impl GsDescBindingStage {
    pub const VERTEX  : GsDescBindingStage = GsDescBindingStage(vk::ShaderStageFlags::VERTEX);
    pub const FRAGMENT: GsDescBindingStage = GsDescBindingStage(vk::ShaderStageFlags::FRAGMENT);
}