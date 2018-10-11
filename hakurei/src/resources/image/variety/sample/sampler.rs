
use ash::vk;
use ash::vk::c_float;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::error::ImageError;

use std::ptr;

pub(crate) struct HaSampler {

    pub handle: vk::Sampler,
}

impl HaSampler {

    pub fn unitialize() -> HaSampler {
        HaSampler {
            handle: vk::Sampler::null(),
        }
    }

    pub fn new(device: &HaDevice, desc: SamplerDescInfo) -> Result<HaSampler, ImageError> {

        let info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SamplerCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::SamplerCreateFlags::empty(),
            mag_filter    : desc.mag_filter,
            min_filter    : desc.min_filter,
            mipmap_mode   : desc.mipmap_mode,
            address_mode_u: desc.address_u,
            address_mode_v: desc.address_v,
            address_mode_w: desc.address_w,
            mip_lod_bias  : desc.mip_lod_bias,
            anisotropy_enable: desc.anisotropy_enable,
            max_anisotropy   : desc.max_anisotropy,
            compare_enable: desc.compare_enable,
            compare_op    : desc.compare_op,
            min_lod: desc.min_lod,
            max_lod: desc.max_lod,
            border_color: desc.border_color,
            unnormalized_coordinates : desc.unnormalize_coordinates,
        };

        let handle = unsafe {
            device.handle.create_sampler(&info, None)
                .or(Err(ImageError::SamplerCreationError))?
        };

        let sampler = HaSampler { handle, };
        Ok(sampler)
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_sampler(self.handle, None);
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct SamplerDescInfo {

    /// mag_filter specifies the magnification filter to apply to lookups.
    pub mag_filter: vk::Filter,
    /// min_filter specifies the minification filter to apply to lookups.
    pub min_filter: vk::Filter,
    /// mipmap_mode specifies the mipmap filter to apply to lookups.
    pub mipmap_mode: vk::SamplerMipmapMode,
    /// address_u specifies the addressing mode for outside [0..1] range for U coordinate.
    pub address_u: vk::SamplerAddressMode,
    /// address_v specifies the addressing mode for outside [0..1] range for V coordinate.
    pub address_v: vk::SamplerAddressMode,
    /// address_w specifies the addressing mode for outside [0..1] range for W coordinate.
    pub address_w: vk::SamplerAddressMode,
    /// mip_lod_bias is the bias to be added to mipmap LOD (level-of-detail) calculation
    /// and bias provided by image sampling functions in SPIR-V.
    pub mip_lod_bias: c_float,
    /// set anisotropy_enable vk::VK_TRUE to enable anisotropic filtering or vk::VK_FALSE to disable it.
    ///
    /// This function needs to enable an physical feature named 'sampler_anisotropy'.
    pub anisotropy_enable: vk::Bool32,
    /// max_anisotropy is the anisotropy value clamp used by the sampler when anisotropy_enable is true.
    ///
    /// If anisotropy_enable is vk::VK_FALSE, max_anisotropy is ignored.
    pub max_anisotropy: c_float,
    /// compare_enable is vk::VK_TRUE to enable comparison against a reference value during lookups, or vk::VK_FALSE otherwise.
    pub compare_enable: vk::Bool32,
    /// compare_op specifies the comparison function to apply to fetched data before filtering as described in the Depth Compare Operation section.
    pub compare_op: vk::CompareOp,
    /// min_lod used to clamp the minimum computed LOD value, as described in the Level-of-Detail Operation section.
    pub min_lod: c_float,
    /// max_lod used to clamp the maxinum computed LOD value, as described in the Level-of-Detail Operation section.
    pub max_lod: c_float,
    /// border_color specifies the predefined border color to use.
    pub border_color: vk::BorderColor,
    /// unnormalize_coordinates controls whether to use unnormalized or normalized texel coordinates to address texels of the image.
    ///
    /// When set to vk::VK_TRUE, the range of the image coordinates used to lookup the texel is in the range of zero to the image dimensions for x, y and z. See specification for more requirement detail.
    ///
    /// When set to vk::VK_FALSE the range of image coordinates is zero to one.
    pub unnormalize_coordinates: vk::Bool32,
}

pub(crate) trait HaSamplerDescAbs {

    fn set_filter(&mut self, mag: vk::Filter, min: vk::Filter);
    fn set_mipmap(&mut self, mode: vk::SamplerMipmapMode, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode, w: vk::SamplerAddressMode);
    fn set_lod(&mut self, min: c_float, max: c_float, mip_bias: c_float);
    fn set_anisotropy(&mut self, max: c_float);
    fn set_compare_op(&mut self, op: vk::CompareOp);
    fn set_border_color(&mut self, color: vk::BorderColor);
    fn set_unnormalize_coordinates_enable(&mut self, enable: bool);
}

impl Default for SamplerDescInfo {

    fn default() -> SamplerDescInfo {
        SamplerDescInfo {
            mag_filter: vk::Filter::Linear,
            min_filter: vk::Filter::Linear,
            mipmap_mode: vk::SamplerMipmapMode::Linear,
            address_u: vk::SamplerAddressMode::Repeat,
            address_v: vk::SamplerAddressMode::Repeat,
            address_w: vk::SamplerAddressMode::Repeat,
            mip_lod_bias: 0.0,
            anisotropy_enable: vk::VK_FALSE,
            max_anisotropy   : 1.0,
            compare_enable: vk::VK_FALSE,
            compare_op    : vk::CompareOp::Always,
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: vk::BorderColor::IntOpaqueBlack,
            unnormalize_coordinates: vk::VK_FALSE,
        }
    }
}