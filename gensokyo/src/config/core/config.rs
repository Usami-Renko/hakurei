
use toml;
use serde_derive::Deserialize;

use gsvk::core::config::CoreConfig;

use crate::config::engine::ConfigMirror;
use crate::config::core::instance::InstanceConfigMirror;
use crate::config::core::validation::ValidationConfigMirror;
use crate::config::core::device::DeviceConfigMirror;
use crate::config::core::physical::PhysicalConfigMirror;
use crate::config::core::swapchain::SwapchainConfigMirror;

use crate::error::GsResult;

#[derive(Deserialize, Default)]
pub(crate) struct CoreConfigMirror {

    instance  : InstanceConfigMirror,
    validation: ValidationConfigMirror,
    device    : DeviceConfigMirror,
    physical  : PhysicalConfigMirror,
    swapchain : SwapchainConfigMirror,
}


impl ConfigMirror for CoreConfigMirror {
    type ConfigType = CoreConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = CoreConfig {
            instance   : self.instance.into_config()?,
            validation : self.validation.into_config()?,
            device     : self.device.into_config()?,
            physical   : self.physical.into_config()?,
            swapchain  : self.swapchain.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        self.instance.parse(toml)?;

        if let Some(v) = toml.get("validation") {
            self.validation.parse(v)?;
        }

        if let Some(v) = toml.get("device") {
            self.device.parse(v)?;
        }

        if let Some(v) = toml.get("physical") {
            self.physical.parse(v)?;
        }

        if let Some(v) = toml.get("swapchain") {
            self.swapchain.parse(v)?;
        }

        Ok(())
    }
}
