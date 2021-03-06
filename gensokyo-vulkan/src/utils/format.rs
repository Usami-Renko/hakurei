
use ash::vk;
use ash::vk::Format as VkFormat;

use crate::types::format::Format;

//macro_rules! parse_config_value_type_func {
//    (bool) => { as_bool };
//}
//
//macro_rules! parse_config {
//    ($variable:tt, $value_type:ty) => {
//        if let Some(v) = toml.get(stringify!($variable)) {
//            self.$variable = v.parse_config_value_type_func!($value_type)
//                .ok_or(ConfigError::ParseError)?
//        }
//    };
//    ($variable:tt) => {
//        if let Some(v) = toml.get(stringify!($variable)) {
//            self.$variable.parse(v)?;
//        }
//    };
//}

macro_rules! raw_str2vk_format {
    ($raw_name:ident, $path_name:ident, {$($raw_content:ident,)*}) => {
        match $raw_name.as_str() {
            $(
                stringify!($raw_content) => Format::any($path_name::$raw_content),
            )*
            _ => panic!(),
        }
    };
}

pub fn vk_string_to_format(raw: &String) -> Format {

    let format = raw_str2vk_format!(raw, VkFormat, {
        UNDEFINED,
        B8G8R8A8_UNORM,
        R8G8B8A8_UNORM,
        D32_SFLOAT,
        D32_SFLOAT_S8_UINT,
        D24_UNORM_S8_UINT,

        R4G4_UNORM_PACK8,
        R4G4B4A4_UNORM_PACK16,
        B4G4R4A4_UNORM_PACK16,
        R5G6B5_UNORM_PACK16,
        B5G6R5_UNORM_PACK16,
        R5G5B5A1_UNORM_PACK16,
        B5G5R5A1_UNORM_PACK16,
        A1R5G5B5_UNORM_PACK16,
        R8_UNORM,
        R8_SNORM,
        R8_USCALED,
        R8_SSCALED,
        R8_UINT,
        R8_SINT,
        R8_SRGB,
        R8G8_UNORM,
        R8G8_SNORM,
        R8G8_USCALED,
        R8G8_SSCALED,
        R8G8_UINT,
        R8G8_SINT,
        R8G8_SRGB,
        R8G8B8_UNORM,
        R8G8B8_SNORM,
        R8G8B8_USCALED,
        R8G8B8_SSCALED,
        R8G8B8_UINT,
        R8G8B8_SINT,
        R8G8B8_SRGB,
        B8G8R8_UNORM,
        B8G8R8_SNORM,
        B8G8R8_USCALED,
        B8G8R8_SSCALED,
        B8G8R8_UINT,
        B8G8R8_SINT,
        B8G8R8_SRGB,
        R8G8B8A8_SNORM,
        R8G8B8A8_USCALED,
        R8G8B8A8_SSCALED,
        R8G8B8A8_UINT,
        R8G8B8A8_SINT,
        R8G8B8A8_SRGB,
        B8G8R8A8_SNORM,
        B8G8R8A8_USCALED,
        B8G8R8A8_SSCALED,
        B8G8R8A8_UINT,
        B8G8R8A8_SINT,
        B8G8R8A8_SRGB,
        A8B8G8R8_UNORM_PACK32,
        A8B8G8R8_SNORM_PACK32,
        A8B8G8R8_USCALED_PACK32,
        A8B8G8R8_SSCALED_PACK32,
        A8B8G8R8_UINT_PACK32,
        A8B8G8R8_SINT_PACK32,
        A8B8G8R8_SRGB_PACK32,
        A2R10G10B10_UNORM_PACK32,
        A2R10G10B10_SNORM_PACK32,
        A2R10G10B10_USCALED_PACK32,
        A2R10G10B10_SSCALED_PACK32,
        A2R10G10B10_UINT_PACK32,
        A2R10G10B10_SINT_PACK32,
        A2B10G10R10_UNORM_PACK32,
        A2B10G10R10_SNORM_PACK32,
        A2B10G10R10_USCALED_PACK32,
        A2B10G10R10_SSCALED_PACK32,
        A2B10G10R10_UINT_PACK32,
        A2B10G10R10_SINT_PACK32,
        R16_UNORM,
        R16_SNORM,
        R16_USCALED,
        R16_SSCALED,
        R16_UINT,
        R16_SINT,
        R16_SFLOAT,
        R16G16_UNORM,
        R16G16_SNORM,
        R16G16_USCALED,
        R16G16_SSCALED,
        R16G16_UINT,
        R16G16_SINT,
        R16G16_SFLOAT,
        R16G16B16_UNORM,
        R16G16B16_SNORM,
        R16G16B16_USCALED,
        R16G16B16_SSCALED,
        R16G16B16_UINT,
        R16G16B16_SINT,
        R16G16B16_SFLOAT,
        R16G16B16A16_UNORM,
        R16G16B16A16_SNORM,
        R16G16B16A16_USCALED,
        R16G16B16A16_SSCALED,
        R16G16B16A16_UINT,
        R16G16B16A16_SINT,
        R16G16B16A16_SFLOAT,
        R32_UINT,
        R32_SINT,
        R32_SFLOAT,
        R32G32_UINT,
        R32G32_SINT,
        R32G32_SFLOAT,
        R32G32B32_UINT,
        R32G32B32_SINT,
        R32G32B32_SFLOAT,
        R32G32B32A32_UINT,
        R32G32B32A32_SINT,
        R32G32B32A32_SFLOAT,
        R64_UINT,
        R64_SINT,
        R64_SFLOAT,
        R64G64_UINT,
        R64G64_SINT,
        R64G64_SFLOAT,
        R64G64B64_UINT,
        R64G64B64_SINT,
        R64G64B64_SFLOAT,
        R64G64B64A64_UINT,
        R64G64B64A64_SINT,
        R64G64B64A64_SFLOAT,
        B10G11R11_UFLOAT_PACK32,
        E5B9G9R9_UFLOAT_PACK32,
        D16_UNORM,
        X8_D24_UNORM_PACK32,
        S8_UINT,
        D16_UNORM_S8_UINT,
        BC1_RGB_UNORM_BLOCK,
        BC1_RGB_SRGB_BLOCK,
        BC1_RGBA_UNORM_BLOCK,
        BC1_RGBA_SRGB_BLOCK,
        BC2_UNORM_BLOCK,
        BC2_SRGB_BLOCK,
        BC3_UNORM_BLOCK,
        BC3_SRGB_BLOCK,
        BC4_UNORM_BLOCK,
        BC4_SNORM_BLOCK,
        BC5_UNORM_BLOCK,
        BC5_SNORM_BLOCK,
        BC6H_UFLOAT_BLOCK,
        BC6H_SFLOAT_BLOCK,
        BC7_UNORM_BLOCK,
        BC7_SRGB_BLOCK,
        ETC2_R8G8B8_UNORM_BLOCK,
        ETC2_R8G8B8_SRGB_BLOCK,
        ETC2_R8G8B8A1_UNORM_BLOCK,
        ETC2_R8G8B8A1_SRGB_BLOCK,
        ETC2_R8G8B8A8_UNORM_BLOCK,
        ETC2_R8G8B8A8_SRGB_BLOCK,
        EAC_R11_UNORM_BLOCK,
        EAC_R11_SNORM_BLOCK,
        EAC_R11G11_UNORM_BLOCK,
        EAC_R11G11_SNORM_BLOCK,
        ASTC_4X4_UNORM_BLOCK,
        ASTC_4X4_SRGB_BLOCK,
        ASTC_5X4_UNORM_BLOCK,
        ASTC_5X4_SRGB_BLOCK,
        ASTC_5X5_UNORM_BLOCK,
        ASTC_5X5_SRGB_BLOCK,
        ASTC_6X5_UNORM_BLOCK,
        ASTC_6X5_SRGB_BLOCK,
        ASTC_6X6_UNORM_BLOCK,
        ASTC_6X6_SRGB_BLOCK,
        ASTC_8X5_UNORM_BLOCK,
        ASTC_8X5_SRGB_BLOCK,
        ASTC_8X6_UNORM_BLOCK,
        ASTC_8X6_SRGB_BLOCK,
        ASTC_8X8_UNORM_BLOCK,
        ASTC_8X8_SRGB_BLOCK,
        ASTC_10X5_UNORM_BLOCK,
        ASTC_10X5_SRGB_BLOCK,
        ASTC_10X6_UNORM_BLOCK,
        ASTC_10X6_SRGB_BLOCK,
        ASTC_10X8_UNORM_BLOCK,
        ASTC_10X8_SRGB_BLOCK,
        ASTC_10X10_UNORM_BLOCK,
        ASTC_10X10_SRGB_BLOCK,
        ASTC_12X10_UNORM_BLOCK,
        ASTC_12X10_SRGB_BLOCK,
        ASTC_12X12_UNORM_BLOCK,
        ASTC_12X12_SRGB_BLOCK,
    });

    format
}

macro_rules! raw_str2phy_feature {
    ($raw_name:ident, $features:ident, {$($raw_content:ident,)*}) => {

        match $raw_name.as_str() {
            $(
                stringify!($raw_content) => {
                    $features.$raw_content = 1;
                },
            )*
            _ => panic!(),
        }
    };
}

pub fn vk_string_to_physical_feature(raw: &String, features: &mut vk::PhysicalDeviceFeatures) {

    raw_str2phy_feature!(raw, features, {
        robust_buffer_access,
        full_draw_index_uint32,
        image_cube_array,
        independent_blend,
        geometry_shader,
        tessellation_shader,
        sample_rate_shading,
        dual_src_blend,
        logic_op,
        multi_draw_indirect,
        draw_indirect_first_instance,
        depth_clamp,
        depth_bias_clamp,
        fill_mode_non_solid,
        depth_bounds,
        wide_lines,
        large_points,
        alpha_to_one,
        multi_viewport,
        sampler_anisotropy,
        texture_compression_etc2,
        texture_compression_astc_ldr,
        texture_compression_bc,
        occlusion_query_precise,
        pipeline_statistics_query,
        vertex_pipeline_stores_and_atomics,
        fragment_stores_and_atomics,
        shader_tessellation_and_geometry_point_size,
        shader_image_gather_extended,
        shader_storage_image_extended_formats,
        shader_storage_image_multisample,
        shader_storage_image_read_without_format,
        shader_storage_image_write_without_format,
        shader_uniform_buffer_array_dynamic_indexing,
        shader_sampled_image_array_dynamic_indexing,
        shader_storage_buffer_array_dynamic_indexing,
        shader_storage_image_array_dynamic_indexing,
        shader_clip_distance,
        shader_cull_distance,
        shader_float64,
        shader_int64,
        shader_int16,
        shader_resource_residency,
        shader_resource_min_lod,
        sparse_binding,
        sparse_residency_buffer,
        sparse_residency_image2_d,
        sparse_residency_image3_d,
        sparse_residency2_samples,
        sparse_residency4_samples,
        sparse_residency8_samples,
        sparse_residency16_samples,
        sparse_residency_aliased,
        variable_multisample_rate,
        inherited_queries,
    });
}
