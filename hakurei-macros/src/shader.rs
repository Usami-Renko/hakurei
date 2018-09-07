
#[macro_export]
macro_rules! define_input {
    (
    #[binding = $binding_index:expr, rate = $input_rate:ident]
    struct $struct_name:ident {
        $(
            #[location = $loc_index:expr, format = $format:ident]
            $filed_name:ident: [$field_type:ty; $element_count:expr],
        )*
    }
    ) => (

        #[derive(Debug, Clone, Copy)]
        struct $struct_name {
            $(
                $filed_name: [$field_type; $element_count],
            )*
        }

        impl $struct_name {

            fn desc() -> VertexInputDescription {
                use std::mem;
                VertexInputDescription {
                    bindings: vec![
                        HaVertexInputBinding {
                            binding: $binding_index,
                            stride: mem::size_of::<Self>() as uint32_t,
                            rate: vertex_rate!($input_rate),
                        },
                    ],
                    attributes: vec![$(
                        HaVertexInputAttribute {
                            binding: $binding_index,
                            location: $loc_index,
                            format: vk_format!($format),
                            offset: offset_of!(Self, $filed_name) as uint32_t,
                        },
                    )*],
                }
            }
        }

    )
}

#[macro_export]
macro_rules! vk_format {
    (float)  => (Format::R32Sfloat);
    (double) => (Format::R64Sfloat);
    (vec2)   => (Format::R32g32Sfloat);
    (vec3)   => (Format::R32g32b32Sfloat);
    (vec4)   => (Format::R32g32b32a32Sfloat);
    (ivec2)  => (Format::R32g32Sint);
    (uvec4)  => (Format::R32g32b32a32Sint);
}

#[macro_export]
macro_rules! vertex_rate {
    (vertex)   => (VertexInputRate::Vertex);
    (instance) => (VertexInputRate::Instance);
}
