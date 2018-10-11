
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::{ HaDevice, HaLogicalDevice };
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::repository::HaImageRepository;
use resources::image::{ ImageDescInfo, ImageViewDescInfo };
use resources::image::{ HaImage, HaImageView };
use resources::image::{ HaSampler, SampleImageInfo, HaSampleImage };
use resources::image::{ DepthStencilImageInfo, HaDepthStencilImage };
use resources::image::{ ImageBarrierBundleAbs, SampleImageBarrierBundle, DepSteImageBarrierBundle };
use resources::image::{ ImageStorageInfo, ImageVarietyType };
use resources::allocator::ImgMemAlloAbstract;
use resources::allocator::{ DeviceImgMemAllocator, CachedImgMemAllocator };
use resources::memory::HaMemoryType;
use resources::command::CommandBufferUsageFlag;
use resources::error::{ ImageError, AllocatorError };

use utility::memory::bind_to_alignment;
use utility::dimension::Dimension2D;

use std::path::Path;
use std::collections::hash_map::{ HashMap, RandomState };

// TODO: Currently not support multi imageview for an image.

pub struct HaImageAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    image_infos: Vec<ImageAllocateInfo>,

    storage_type: ImageStorageType,
    allocator: Box<ImgMemAlloAbstract>,
    require_mem_flag: vk::MemoryPropertyFlags,
    memory_selector : MemorySelector,
}

impl HaImageAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, ty: ImageStorageType) -> HaImageAllocator {

        HaImageAllocator {

            physical: physical.clone(),
            device  : device.clone(),

            image_infos: vec![],

            storage_type: ty,
            allocator: ty.allocator(),
            require_mem_flag: ty.memory_type().property_flags(),
            memory_selector : MemorySelector::init(physical),
        }
    }

    pub fn attach_sample_image(&mut self, path: &Path, info: SampleImageInfo) -> Result<HaSampleImage, AllocatorError> {

        let storage = ImageStorageInfo::from_load2d(path)?;
        let image = HaImage::config(&self.device, &info.image_desc, storage.dimension, storage.format)?;
        self.memory_selector.try(image.requirement.memory_type_bits, self.require_mem_flag)?;

        let sampler = HaSampler::new(&self.device, info.sampler_desc)?;
        let result = HaSampleImage::setup(sampler, info.binding, info.count, self.image_infos.len());

        let image_info = ImageAllocateInfo::new(ImageVarietyType::SampleImage(info.pipeline_stage), storage, image, info.image_desc, info.view_desc);
        self.image_infos.push(image_info);

        Ok(result)
    }

    pub fn attach_depth_stencil_image(&mut self, info: DepthStencilImageInfo, dimension: Dimension2D) -> Result<HaDepthStencilImage, AllocatorError> {

        let storage = ImageStorageInfo::from_unload(dimension, info.usage.dst_format(&self.physical));
        let image = HaImage::config(&self.device, &info.image_desc, storage.dimension, storage.format)?;
        self.memory_selector.try(image.requirement.memory_type_bits, self.require_mem_flag)?;

        let result = HaDepthStencilImage::setup(info.binding, info.count, self.image_infos.len(), storage.format);

        let mut view_desc = info.view_desc;
        view_desc.reset_depth_image_aspect_mask(storage.format);

        let image_info = ImageAllocateInfo::new(ImageVarietyType::DepthStencilImage(info.usage), storage, image, info.image_desc, view_desc);
        self.image_infos.push(image_info);

        Ok(result)
    }

    pub fn allocate(&mut self) -> Result<HaImageRepository, AllocatorError> {

        if self.image_infos.is_empty() {
            return Err(AllocatorError::Image(ImageError::NoImageAttachError))
        }

        // 1.create image buffer and memories.
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);
        let total_space = self.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        self.allocator.allocate(
            &self.device, total_space, optimal_memory_index, Some(mem_type)
        )?;

        {
            let memory = self.allocator.borrow_memory()?;

            // bind images to memory.
            let mut offset = 0;
            for image_info in self.image_infos.iter() {
                memory.bind_to_image(&self.device, &image_info.image, offset)?;
                offset += image_info.space;
            }
        }

        // 2.create image view for each image.
        let mut views = vec![];
        for image_info in self.image_infos.iter() {
            views.push(image_info.generate_view(&self.device)?);
        }

        // 3.create command buffer.
        let mut transfer = HaLogicalDevice::transfer(&self.device);

        let mut barrier_bundles = {

            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record();
            recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

            // 4. make image barrier transitions.
            let mut barrier_bundles = collect_barrier_bundle(&self.physical, &self.device, &self.image_infos);
            for bundle in barrier_bundles.iter_mut() {
                bundle.make_transfermation(&recorder, &self.image_infos)?;
            }

            // 5.submit command buffer.
            recorder.end_record()?;

            barrier_bundles
        };

        // 6.execute the command.
        transfer.excute()?;

        barrier_bundles.iter_mut()
            .for_each(|bundle| bundle.cleanup());

        // clear the image_infos, and give the images ownership to HaImageRepository.
        let images = self.image_infos.drain(..)
            .map(|info| info.image).collect::<Vec<_>>();

        // final done.
        let repository = HaImageRepository::store(&self.device, images, views, self.allocator.take_memory()?);
        Ok(repository)
    }

    pub fn reset(&mut self) {

        self.image_infos.iter().for_each(|image_info| {
            image_info.cleanup(&self.device);
        });

        self.memory_selector.reset();
        self.require_mem_flag = self.storage_type.memory_type().property_flags();
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {
    Device,
    Cached,
}

impl ImageStorageType {

    fn allocator(&self) -> Box<ImgMemAlloAbstract> {
        match self {
            | ImageStorageType::Device => Box::new(DeviceImgMemAllocator::new()),
            | ImageStorageType::Cached => Box::new(CachedImgMemAllocator::new()),
        }
    }

    fn memory_type(&self) -> HaMemoryType {
        match self {
            | ImageStorageType::Cached  => HaMemoryType::CachedMemory,
            | ImageStorageType::Device  => HaMemoryType::DeviceMemory,
        }
    }
}


pub(crate) struct ImageAllocateInfo {

    type_: ImageVarietyType,

    pub(crate) image: HaImage,
    pub(crate) _image_desc: ImageDescInfo,
    pub(crate) view_desc : ImageViewDescInfo,

    pub(crate) storage   : ImageStorageInfo,
    pub(crate) space     : vk::DeviceSize,
}

impl ImageAllocateInfo {

    fn new(type_: ImageVarietyType, storage: ImageStorageInfo, image: HaImage, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> ImageAllocateInfo {

        let space = bind_to_alignment(image.requirement.size, image.requirement.alignment);

        ImageAllocateInfo {
            type_, image, _image_desc: image_desc, view_desc, storage, space,
        }
    }

    fn generate_view(&self, device: &HaDevice) -> Result<HaImageView, ImageError> {

        let view = HaImageView::config(device, &self.image, &self.view_desc, self.storage.format)?;
        Ok(view)
    }

    fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image(self.image.handle, None);
        }
    }
}

fn collect_barrier_bundle(physical: &HaPhyDevice, device: &HaDevice, image_infos: &[ImageAllocateInfo]) -> Vec<Box<ImageBarrierBundleAbs>> {

    let mut barrier_indices: HashMap<ImageVarietyType, Vec<usize>, RandomState> = HashMap::new();

    for (index, image_info) in image_infos.iter().enumerate() {

        // make the logic a little strange to avoid borrow conflict.
        let is_found = {
            if let Some(indices) = barrier_indices.get_mut(&image_info.type_) {
                indices.push(index);
                true
            } else {
                false
            }
        };

        if is_found == false {
            barrier_indices.insert(image_info.type_, vec![index]);
        }
    };

    let bundles = barrier_indices.into_iter()
        .map(|(image_type, indices)| {

        match image_type {
            | ImageVarietyType::SampleImage(stage) => {
                let bundle = Box::new(SampleImageBarrierBundle::new(physical, device, stage.clone(), indices));
                bundle as Box<ImageBarrierBundleAbs>
            },
            | ImageVarietyType::DepthStencilImage(usage) => {
                let bundle = Box::new(DepSteImageBarrierBundle::new(usage.clone(), indices));
                bundle as Box<ImageBarrierBundleAbs>
            },
        }

    }).collect::<Vec<_>>();

    bundles
}