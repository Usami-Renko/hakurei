
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::BufferInstance;
use buffer::allocator::HaBufferAllocator;
use buffer::allocator::types::BufferStorageType;
use buffer::instance::{ HaImgsrcBlock, ImgsrcBlockInfo };
use buffer::HaBufferRepository;

use image::barrier::HaImageBarrier;
use image::storage::ImageSource;
use image::enums::ImagePipelineStage;
use image::instance::traits::ImageBarrierBundleAbs;
use image::allocator::ImageAllocateInfo;

use memory::transfer::DataCopyer;
use memory::types::Staging;
use memory::AllocatorError;

pub struct SampleImageBarrierBundle {

    info_indices: Vec<usize>,
    dst_stage: ImagePipelineStage,

    staging_repository: Option<HaBufferRepository<Staging>>,
}

impl ImageBarrierBundleAbs for SampleImageBarrierBundle {

    fn make_transfermation(&mut self, physical: &HaPhyDevice, device: &HaDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        // create staging buffer and memories
        let (mut staging_repository, buffer_blocks) = self.create_staging_repository(physical, device, infos)?;
        // send textures to the staging buffer
        self.upload_staging_data(&mut staging_repository, &buffer_blocks, infos)?;

        // make image barrier transition for data transfer.
        let transfer_barriers = self.info_indices.iter()
            .map(|&index| self.transfer_barrier(&mut infos[index])).collect();
        copyer.recorder().image_pipeline_barrrier(
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(), // dependencies specifying how execution and memory dependencies are formed.
            transfer_barriers
        );

        // copy buffer to image.
        for (i, &index) in self.info_indices.iter().enumerate() {
            copyer.copy_buffer_to_image(buffer_blocks[i].as_block_ref(), &infos[index]);
        }

        // make image barrier transition for final layout.
        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&mut infos[index])).collect();
        let _ = copyer.recorder().image_pipeline_barrrier(
            vk::PipelineStageFlags::TRANSFER,
            self.dst_stage.to_raw_flag(),
            vk::DependencyFlags::empty(),
            final_barriers
        );

        self.staging_repository = Some(staging_repository);

        Ok(())
    }

    fn cleanup(&mut self) {
        if let Some(ref mut repository) = self.staging_repository {
            repository.cleanup();
        }
    }
}

impl SampleImageBarrierBundle {

    pub fn new(dst_stage: ImagePipelineStage, indices: Vec<usize>) -> SampleImageBarrierBundle {
        SampleImageBarrierBundle {

            info_indices: indices, dst_stage,
            staging_repository: None,
        }
    }

    fn create_staging_repository(&mut self, physical: &HaPhyDevice, device: &HaDevice, infos: &Vec<ImageAllocateInfo>) -> Result<(HaBufferRepository<Staging>, Vec<HaImgsrcBlock>), AllocatorError> {

        let mut staging_indices = vec![];

        let mut staging_allocator = HaBufferAllocator::new(physical, device, BufferStorageType::STAGING);

        for &index in self.info_indices.iter() {
            let img_info = ImgsrcBlockInfo::new(infos[index].space);
            let buffer_index = staging_allocator.append_buffer(img_info)?;
            staging_indices.push(buffer_index);
        }

        let distributor = staging_allocator.allocate()?;

        let mut staging_buffers = vec![];
        for index in staging_indices.into_iter() {
            let staging_buffer = distributor.acquire_imgsrc(index);
            staging_buffers.push(staging_buffer);
        }

        Ok((distributor.into_repository(), staging_buffers))
    }

    fn upload_staging_data(&self, staging_repository: &mut HaBufferRepository<Staging>, img_data_blocks: &[HaImgsrcBlock], infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let mut uploader = staging_repository.data_uploader()?;

        for (&info_index, img_block) in self.info_indices.iter().zip(img_data_blocks.iter()) {

            match infos[info_index].storage.source {
                | ImageSource::UploadData(ref source) => {
                    uploader.upload(img_block, &source.data)?;
                },
                | _ => panic!(),
            }
        }

        uploader.finish()?;

        Ok(())
    }

    fn transfer_barrier(&self, info: &mut ImageAllocateInfo) -> HaImageBarrier {

        info.final_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;

        HaImageBarrier::new(&info.image, info.view_desc.subrange)
            .access_mask(
                vk::AccessFlags::empty(),
                vk::AccessFlags::TRANSFER_WRITE)
            .layout(info.image_desc.property.initial_layout, info.final_layout)
            .build()
    }

    fn final_barrier(&self, info: &mut ImageAllocateInfo) -> HaImageBarrier {

        let previous_layout = info.final_layout;
        info.final_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;

        HaImageBarrier::new(&info.image, info.view_desc.subrange)
            .access_mask(
                vk::AccessFlags::TRANSFER_WRITE,
                vk::AccessFlags::SHADER_READ)
            .layout(previous_layout, info.final_layout)
            .build()
    }
}
