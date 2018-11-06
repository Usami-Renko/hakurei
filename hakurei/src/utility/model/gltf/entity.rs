
use gltf;
use ash::vk::uint32_t;

use utility::model::{ GltfResources, GltfRawData };
use utility::model::GltfScene;
use utility::model::GltfHierarchyAbstract;
use utility::model::{ ModelLoadingErr, ModelGltfLoadingError };

use resources::buffer::{ HaIndexBlock, HaVertexBlock };
use resources::allocator::BufferStorageType;
use resources::repository::HaBufferRepository;
use resources::command::{ HaCommandRecorder, CmdVertexBindingInfo, CmdIndexBindingInfo };
use resources::toolkit::AllocatorKit;
use resources::error::AllocatorError;
use pipeline::shader::VertexInputDescription;

use std::path::Path;

#[derive(Default)]
pub struct GltfEntity {

    _scenes: Vec<GltfScene>,
    resources: GltfResources,

    allo_res: Option<AllocateResource>,
}

impl GltfEntity {

    pub(crate) fn load(path: impl AsRef<Path>) -> Result<GltfEntity, ModelLoadingErr> {

        let mut resources = GltfResources::default();

        let (document, buffers, images) = gltf::import(path)
            .map_err(|e| ModelLoadingErr::Gltf(ModelGltfLoadingError::Gltf(e)))?;
        let raw_data = GltfRawData {
            document, buffers, images,
        };

        let mut scenes = vec![];
        for raw_scene in raw_data.document.scenes() {
            let scene = GltfScene::from_hierarchy(raw_scene, &mut resources, &raw_data)
                .map_err(|e| ModelLoadingErr::Gltf(e))?;
            scenes.push(scene);
        }

        let entity = GltfEntity {
            _scenes: scenes, resources, allo_res: None,
        };

        Ok(entity)
    }

    pub fn config_buffer(&mut self, kit: &AllocatorKit, storage: BufferStorageType) -> Result<(), AllocatorError> {

        let mut allocator = kit.buffer(storage);

        let mut vertex_buffers = vec![];
        let mut index_buffers = vec![];
        let mut index_counts = vec![];

        // create a buffer for each primitive.
        for mesh in self.resources.meshes.iter() {
            for primitive in mesh.primitives.iter() {

                let block_info = primitive.block_info();
                let vertex_buffer = allocator.append_vertex(block_info)?;
                vertex_buffers.push(vertex_buffer);

                let index_info = primitive.index_info();
                let index_buffer = allocator.append_index(index_info)?;
                index_buffers.push(index_buffer);
            }
        }

        let mut repository = allocator.allocate()?;

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

            uploader.done()?;
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

    pub fn record_command(&self, recorder: &HaCommandRecorder) {

        // TODO: handle unwrap().
        let res = self.allo_res.as_ref().unwrap();

        let element_count = res.vertexs.len();
        for i in 0..element_count {

            let vertex_buffer = &res.vertexs[i];
            let index_buffer = &res.indices[i];

            recorder
                .bind_vertex_buffers(0,&[CmdVertexBindingInfo { block: vertex_buffer, sub_block_index: None }])
                .bind_index_buffer(CmdIndexBindingInfo { block: index_buffer, sub_block_index: None })
                .draw_indexed(res.index_counts[i] as uint32_t, 1, 0, 0, 0);
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref mut res) = self.allo_res {
            res.repository.cleanup();
        }
    }

    pub fn vertex_desc(&self) -> VertexInputDescription {
        Vertex::desc()
    }
}

struct AllocateResource {

    vertexs: Vec<HaVertexBlock>,
    indices: Vec<HaIndexBlock>,
    index_counts: Vec<usize>,
    repository: HaBufferRepository,
}


// TODO: Remove the following codes.
use pipeline::shader::{ VertexInputRate, HaVertexInputBinding, HaVertexInputAttribute };
use ash::vk::Format;
define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec3]
        position : [f32; 3],
    }
}
