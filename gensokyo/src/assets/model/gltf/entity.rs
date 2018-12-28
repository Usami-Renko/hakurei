
use ash::vk;
use gltf;

use gsvk::buffer::GsBufferRepository;
use gsvk::buffer::instance::{ GsVertexBlock, GsIndexBlock };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use gsvk::pipeline::shader::{ VertexInputDescription, GsVertexInputAttribute, GsVertexInputBinding };
use gsvk::command::GsCommandRecorder;
use gsvk::memory::AllocatorError;

use gsvk::types::vkuint;
use gsma::{ define_input, vertex_rate, vk_format, offset_of };

use crate::assets::model::{ GltfResources, GltfRawData };
use crate::assets::model::GltfScene;
use crate::assets::model::GltfHierarchyAbstract;
use crate::assets::model::{ModelLoadingError, ModelGltfLoadingError };

use crate::toolkit::AllocatorKit;

use std::path::Path;

#[derive(Default)]
pub struct GltfEntity<M> where M: BufferMemoryTypeAbs + Copy {

    phantom_type: M,

    _scenes: Vec<GltfScene>,
    resources: GltfResources,

    allo_res: Option<AllocateResource<M>>,
}

impl<M> GltfEntity<M> where M: BufferMemoryTypeAbs + Copy {

    pub(crate) fn load(path: impl AsRef<Path>, typ: M) -> Result<GltfEntity<M>, ModelLoadingError> {

        let mut resources = GltfResources::default();

        let (document, buffers, images) = gltf::import(path)
            .map_err(|e| ModelLoadingError::Gltf(ModelGltfLoadingError::Gltf(e)))?;
        let raw_data = GltfRawData {
            document, buffers, images,
        };

        let mut scenes = vec![];
        for raw_scene in raw_data.document.scenes() {
            let scene = GltfScene::from_hierarchy(raw_scene, &mut resources, &raw_data)
                .map_err(|e| ModelLoadingError::Gltf(e))?;
            scenes.push(scene);
        }

        let entity = GltfEntity {
            phantom_type: typ,
            _scenes: scenes, resources, allo_res: None,
        };

        Ok(entity)
    }

    pub fn config_buffer(&mut self, kit: &AllocatorKit) -> Result<(), AllocatorError> {

        let mut allocator = kit.buffer(self.phantom_type);

        let mut vertex_indices = vec![];
        let mut index_indices  = vec![];

        // create a buffer for each primitive.
        for mesh in self.resources.meshes.iter() {
            for primitive in mesh.primitives.iter() {

                let block_info = primitive.block_info();
                let vertex_index = allocator.append_buffer(block_info)?;
                vertex_indices.push(vertex_index);

                let index_info = primitive.index_info();
                let index_index = allocator.append_buffer(index_info)?;
                index_indices.push(index_index);
            }
        }

        let distributor = allocator.allocate()?;

        let mut vertex_buffers = vec![];
        let mut index_buffers = vec![];
        let mut index_counts = vec![];

        for vertex in vertex_indices.into_iter() {
            let vertex_buffer = distributor.acquire_vertex(vertex);
            vertex_buffers.push(vertex_buffer);
        }
        for index in index_indices.into_iter() {
            let index_buffer = distributor.acquire_index(index);
            index_buffers.push(index_buffer);
        }

        let mut repository = distributor.into_repository();

        {
            let mut uploader = repository.data_uploader()?;

            let mut buffer_index = 0;
            for mesh in self.resources.meshes.iter() {
                for primitive in mesh.primitives.iter() {

                    primitive.upload_vertex_data(&vertex_buffers[buffer_index], &mut uploader)?;
                    primitive.upload_index_data(&index_buffers[buffer_index], &mut uploader)?;

                    index_counts.push(primitive.index_count());

                    buffer_index += 1;
                }
            }

            uploader.finish()?;
        }

        let res = AllocateResource {
            vertexs: vertex_buffers,
            indices: index_buffers,
            index_counts,
            repository,
        };
        self.allo_res = Some(res);

        Ok(())
    }

    pub fn record_command(&self, recorder: &GsCommandRecorder) {

        // TODO: handle unwrap().
        let res = self.allo_res.as_ref().unwrap();

        let element_count = res.vertexs.len();
        for i in 0..element_count {

            let vertex_buffer = &res.vertexs[i];
            let index_buffer = &res.indices[i];

            recorder
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_index_buffer(index_buffer, 0)
                .draw_indexed(res.index_counts[i] as vkuint, 1, 0, 0, 0);
        }
    }

    pub fn vertex_desc(&self) -> VertexInputDescription {
        Vertex::desc()
    }
}

struct AllocateResource<M> where M: BufferMemoryTypeAbs + Copy {

    vertexs: Vec<GsVertexBlock>,
    indices: Vec<GsIndexBlock>,
    index_counts: Vec<usize>,
    #[allow(dead_code)]
    repository: GsBufferRepository<M>,
}


// TODO: Remove the following codes.
define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec3]
        position : [f32; 3],
    }
}
