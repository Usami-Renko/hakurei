
use crate::assets::gltf::storage::{ GltfRawDataAgency, GltfShareResource };
use crate::assets::gltf::traits::{ GsGltfHierachy, GltfHierachyIndex, GltfHierachyInstance };
use crate::assets::gltf::mesh::{ GsGltfMesh, GltfMeshIndex, GltfMeshInstance, GltfMeshVerification };
use crate::assets::gltf::material::GltfShareResourceTmp;
use crate::assets::gltf::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferDistributor };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::GsUniformBlock;
use gsvk::memory::transfer::{ GsBufferDataUploader, GsBufferDataUpdater };
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;

/// A wrapper class for node level in glTF, containing the data read from glTF file.
pub(super) struct GsGltfNode {

    mesh: Option<GsGltfMesh>,
    transform: Matrix4F,

    children: Vec<Box<GsGltfNode>>,
}

impl GsGltfNode {

    /// Apply parent node's transformation to current node level.
    fn combine_transform(&mut self, parent_transform: &Matrix4F) {
        self.transform = self.transform * parent_transform;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct GltfNodeVerification {

    verification: GltfMeshVerification,
}

pub(super) struct GltfNodeIndex {

    root_index: Option<GltfMeshIndex>,
    children_indices: Vec<Box<GltfNodeIndex>>,
}

pub(super) struct GltfNodeInstance {

    mesh: Option<GltfMeshInstance>,
    children: Vec<Box<GltfNodeInstance>>,
}

impl<'a> GsGltfHierachy<'a> for GsGltfNode {
    type HierachyRawType    = gltf::Node<'a>;
    type HierachyVerifyType = GltfNodeVerification;
    type HierachyIndex      = GltfNodeIndex;
    type HierachyTransform  = Matrix4F;

    fn from_hierachy(hierachy: Self::HierachyRawType, agency: &GltfRawDataAgency, res: &mut GltfShareResourceTmp) -> Result<Self, GltfError> {

        let transform = Matrix4F::from(hierachy.transform().matrix());

        let mut children = vec![];
        for child_node in hierachy.children() {
            let mut sub_node = GsGltfNode::from_hierachy(child_node, &agency, res)?;
            sub_node.combine_transform(&transform);
            children.push(Box::new(sub_node));
        }

        let mesh = if let Some(raw_mesh) = hierachy.mesh() {
            Some(GsGltfMesh::from_hierachy(raw_mesh, &agency, res)?)
        } else {
            None
        };

        let target = GsGltfNode { mesh, transform, children };
        Ok(target)
    }

    fn generate_verification(&self) -> Option<Self::HierachyVerifyType> {

        self.mesh.as_ref().and_then(|mesh| {
            mesh.generate_verification()
                .and_then(|verification| Some(GltfNodeVerification { verification }))
        }).or_else(|| {
            self.children.iter()
                .find_map(|child| child.generate_verification())
        })
    }

    fn verify(&self, verification: &Self::HierachyVerifyType) -> bool {

        let is_mesh_verified = self.mesh.as_ref()
            .and_then(|m| Some(m.verify(&verification.verification)))
            .unwrap_or(true);

        if is_mesh_verified {
            self.children.iter().all(|child| child.verify(verification))
        } else {
            false
        }
    }

    fn apply_transform(&mut self, transform: &Self::HierachyTransform) {

        if let Some(ref mut mesh) = self.mesh {
            mesh.apply_transform(&self.transform);
        }

        self.children.iter_mut().for_each(|child_node| {
            child_node.apply_transform(transform);
        });
    }

    fn allocate<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Self::HierachyIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let root_index = if let Some(ref mesh) = self.mesh {
            Some(mesh.allocate(allocator)?)
        } else {
            None
        };

        let mut children_indices = vec![];
        for child_node in self.children.iter() {
            let child_index = child_node.allocate(allocator)?;
            children_indices.push(Box::new(child_index));
        }

        let target = GltfNodeIndex { root_index, children_indices };
        Ok(target)
    }
}

impl GltfHierachyIndex for GltfNodeIndex {
    type HierachyInstance = GltfNodeInstance;

    fn distribute<M>(self, distributor: &GsBufferDistributor<M>) -> Self::HierachyInstance
        where M: BufferMemoryTypeAbs {

        let mesh = if let Some(index) = self.root_index {
            Some(index.distribute(distributor))
        } else {
            None
        };

        let mut children = vec![];
        for child_index in self.children_indices.into_iter() {
            let child = child_index.distribute(distributor);
            children.push(Box::new(child));
        }

        GltfNodeInstance { mesh, children }
    }
}

impl GltfHierachyInstance for GltfNodeInstance {
    type HierachyDataType = GsGltfNode;

    fn upload(&self, uploader: &mut GsBufferDataUploader, data: &Self::HierachyDataType) -> Result<(), AllocatorError> {

        if let Some(ref mesh) = self.mesh {
            if let Some(ref mesh_data) = data.mesh {
                mesh.upload(uploader, mesh_data)?;
            } else {
                unreachable!()
            }
        }

        for (child_node, child_data) in self.children.iter().zip(data.children.iter()) {
            child_node.upload(uploader, child_data)?;
        }

        Ok(())
    }

    fn update_uniform(&self, updater: &mut GsBufferDataUpdater, to: &GsUniformBlock, res: &GltfShareResource) -> Result<(), AllocatorError> {

        if let Some(ref mesh) = self.mesh {
            mesh.update_uniform(updater, to, res)?;
        }

        for child_node in self.children.iter() {
            child_node.update_uniform(updater, to, res)?;
        }

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        self.mesh.as_ref().map(|mesh| mesh.record_command(recorder));

        self.children.iter().for_each(|child_node| {
            child_node.record_command(recorder);
        });
    }
}
