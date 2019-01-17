
use std::rc::Rc;
pub type GsChain = Rc<self::chain::GsSwapchain>;

pub use self::chain::{ GsSwapchain, SwapchainConfig };
pub use self::builder::SwapchainBuilder;

mod chain;
mod builder;
mod support;
