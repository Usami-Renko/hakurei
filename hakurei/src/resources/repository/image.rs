
use core::device::HaDevice;

use resources::memory::{ HaDeviceMemory, HaMemoryAbstract };
use resources::image::{ HaImage, HaImageView, ImageViewItem };

pub struct HaImageRepository {

    device : Option<HaDevice>,
    images : Vec<HaImage>,
    views  : Vec<HaImageView>,
    memory : Option<HaDeviceMemory>,
}

impl HaImageRepository {

    pub fn empty() -> HaImageRepository {
        HaImageRepository {

            device: None,
            images: vec![],
            views : vec![],
            memory: None,
        }
    }

    pub(crate) fn store(device: &HaDevice, images: Vec<HaImage>, views: Vec<HaImageView>, memory: HaDeviceMemory) -> HaImageRepository {

        HaImageRepository {
            device: Some(device.clone()),
            memory: Some(memory),
            images, views,
        }
    }

    pub(crate) fn view_at(&self, item: &ImageViewItem) -> &HaImageView {
        &self.views[item.view_index]
    }

    pub fn view_item(&self, view_index: usize) -> ImageViewItem {
        ImageViewItem {
            view_index,
            image_handle: self.images[view_index].handle,
            view_handle : self.views[view_index].handle,
        }
    }

    pub fn cleanup(&mut self) {

        if let Some(ref device) = self.device {
            for image in self.images.iter() {
                image.cleanup(device);
            }
            for view in self.views.iter() {
                view.cleanup(device);
            }

            if let Some(ref memory) = self.memory {
                memory.cleanup(device);
            }
        }

        self.views.clear();
        self.images.clear();
    }
}
