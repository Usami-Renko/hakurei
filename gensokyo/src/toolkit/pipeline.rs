
use crate::error::{ GsResult, GsError };

use gsvk::core::device::GsDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::pipeline::graphics::{ GraphicsPipelineBuilder, GraphicsPipelineConfig };
use gsvk::pipeline::pass::{ GsRenderPass, RenderPassBuilder };
use gsvk::pipeline::pass::{ RenderAttachment, RenderAttachmentPrefab };
use gsvk::pipeline::pass::{ RenderDependency, SubpassStage };
use gsvk::pipeline::shader::{ GsShaderInfo, VertexInputDescription };

pub struct PipelineKit {

    device: GsDevice,
    chain : GsChain,
}

impl PipelineKit {

    pub(crate) fn init(device: &GsDevice, chain: &GsChain) -> PipelineKit {

        PipelineKit {
            device : device.clone(),
            chain  : chain.clone(),
        }
    }

    pub fn pass_builder(&self) -> RenderPassBuilder {

        RenderPassBuilder::new(&self.device, &self.chain)
    }

    pub fn graphics_pipeline_builder(&self) -> GsResult<GraphicsPipelineBuilder> {

        GraphicsPipelineBuilder::new(&self.device).map_err(GsError::from)
    }

    pub fn pipeline_config(&self, shaders: impl Into<Vec<GsShaderInfo>>, input: VertexInputDescription, render_pass: GsRenderPass) -> GraphicsPipelineConfig {
        GraphicsPipelineConfig::new(shaders, input, render_pass, self.chain.extent())
    }

    pub fn present_attachment(&self) -> RenderAttachment {
        RenderAttachment::setup(RenderAttachmentPrefab::PresentAttachment, self.chain.format())
    }

    pub fn subpass_dependency(&self, src: SubpassStage, dst: SubpassStage) -> RenderDependency {
        RenderDependency::setup(src, dst)
    }
}
