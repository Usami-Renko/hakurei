
pub use self::input::{ GsVertexInputAttribute, GsVertexInputBinding, VertexInputDescription };
pub use self::module::GsShaderCI;

pub(super) use self::module::GsShaderModule;

pub(super) mod shaderc;

mod module;
mod input;
mod specialization;
