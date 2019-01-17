
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;
use crate::error::{ VkResult, VkError };

use std::ptr;

pub struct GsSemaphore {

    pub(crate) handle: vk::Semaphore,
    device: GsDevice,
}

impl GsSemaphore {

    pub fn setup(device: &GsDevice) -> VkResult<GsSemaphore> {

        let create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let handle = unsafe {
            device.handle.create_semaphore(&create_info, None)
                .or(Err(VkError::create("Semaphore")))?
        };

        let semaphore = GsSemaphore {
            device: device.clone(),
            handle,
        };
        Ok(semaphore)
    }

    pub fn destroy(&self) {
        unsafe {
            self.device.handle.destroy_semaphore(self.handle, None);
        }
    }
}
