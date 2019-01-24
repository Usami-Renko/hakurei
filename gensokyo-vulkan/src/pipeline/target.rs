
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::pipeline::layout::GsPipelineLayout;
use crate::pipeline::pass::GsRenderPass;

use crate::utils::phantom::{ Graphics, Compute };

use std::marker::PhantomData;
use std::rc::Rc;
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign };

// -------------------------------------------------------------------------------------
pub struct GsPipeline<T> where T: GsVkPipelineType {

    phantom_type: PhantomData<T>,

    pub(crate) handle: vk::Pipeline,
    pub(crate) pass  : Rc<GsRenderPass>,
    pub(crate) layout: Rc<GsPipelineLayout>,

    device: GsDevice,
}

impl<T> GsPipeline<T> where T: GsVkPipelineType {

    pub(super) fn new(device: &GsDevice, handle: vk::Pipeline, layout: Rc<GsPipelineLayout>, pass: Rc<GsRenderPass>) -> GsPipeline<T> {

        GsPipeline {
            phantom_type: PhantomData,
            device: device.clone(),
            layout, handle, pass,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.pass.frame_count()
    }

    pub fn render_pass_ref(&self) -> &Rc<GsRenderPass> {
        &self.pass
    }

    pub fn destroy(&self) {

        // TODO: Fix destroy.
    }
}

impl<T> Drop for GsPipeline<T> where T: GsVkPipelineType {

    fn drop(&mut self) {
        unsafe {
            self.device.handle.destroy_pipeline(self.handle, None);
        }

        if Rc::strong_count(&self.layout) == 1 {
            self.layout.destroy(&self.device);
        }

        if Rc::strong_count(&self.pass) == 1 {
            self.pass.destroy(&self.device);
        }
    }
}
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
pub trait GsVkPipelineType {
    const BIND_POINT: ash::vk::PipelineBindPoint;
}

impl GsVkPipelineType for Graphics {
    const BIND_POINT: vk::PipelineBindPoint = vk::PipelineBindPoint::GRAPHICS;
}

impl GsVkPipelineType for Compute {
    const BIND_POINT: vk::PipelineBindPoint = vk::PipelineBindPoint::COMPUTE;
}
// -------------------------------------------------------------------------------------


// -------------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GsPipelineStage(pub(crate) vk::ShaderStageFlags);

impl GsPipelineStage {
    pub const VERTEX                 : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::VERTEX);
    pub const TESSELLATION_CONTROL   : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::TESSELLATION_CONTROL);
    pub const TESSELLATION_EVALUATION: GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::TESSELLATION_EVALUATION);
    pub const GEOMETRY               : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::GEOMETRY);
    pub const FRAGMENT               : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::FRAGMENT);
    pub const COMPUTE                : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::COMPUTE);
}

impl BitAnd for GsPipelineStage {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GsPipelineStage(self.0 & rhs.0)
    }
}

impl BitAndAssign for GsPipelineStage {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for GsPipelineStage {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self {
        GsPipelineStage(self.0 | rhs.0)
    }
}

impl BitOrAssign for GsPipelineStage {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// -------------------------------------------------------------------------------------
