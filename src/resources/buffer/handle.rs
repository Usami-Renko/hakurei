
use ash;
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::BufferUsageFlag;
use resources::buffer::BufferCreateFlag;
use resources::error::BufferError;
use resources::memory::MemoryPropertyFlag;

use utility::marker::VulkanFlags;

use std::ptr;
use std::mem;

pub(crate) struct HaBuffer {

    pub(crate) handle : vk::Buffer,
    requirement       : vk::MemoryRequirements,
}

pub struct BufferConfig<'flag> {

    pub estimate_size: vk::DeviceSize,
    pub usages       : &'flag [BufferUsageFlag],
    // TODO: Turn the flags into bool options.
    pub buffer_flags : &'flag [BufferCreateFlag],
    pub memory_flags : &'flag [MemoryPropertyFlag],
}

impl HaBuffer {

    /// Generate a buffer object.
    ///
    /// estimate_size is the size in bytes of the buffer to be created. size must be greater than 0.
    ///
    /// If the buffer is accessed by one queue family, set sharing_queue_families to None,
    /// or set it the queue family indices to share accessing.
    pub fn generate(device: &HaLogicalDevice, estimate_size: vk::DeviceSize, usages: &[BufferUsageFlag], flags: &[BufferCreateFlag], sharing_queue_families: Option<Vec<uint32_t>>)
        -> Result<HaBuffer, BufferError> {

        let (sharing_mode, indices) = match sharing_queue_families {
            | Some(families) => (vk::SharingMode::Concurrent, families),
            | None           => (vk::SharingMode::Exclusive, vec![]),
        };

        let create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags : flags.flags(),
            size  : estimate_size,
            usage : usages.flags(),
            sharing_mode,
            queue_family_index_count: indices.len() as uint32_t,
            p_queue_family_indices  : indices.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_buffer(&create_info, None)
                .or(Err(BufferError::BufferCreationError))?
        };

        let requirement = device.handle.get_buffer_memory_requirements(handle);

        let buffer = HaBuffer {
            handle,
            requirement,
        };
        Ok(buffer)
    }

    pub fn copy_data<D: Copy>(&self, data_ptr: *mut vk::c_void, size: vk::DeviceSize, data: &Vec<D>) {

        let mut vert_algn = unsafe {
            ash::util::Align::new(
                data_ptr,
                mem::align_of::<D>() as vk::DeviceSize,
                size,
            )
        };

        vert_algn.copy_from_slice(data);
    }

    pub fn require_memory_size(&self) -> vk::DeviceSize {
        self.requirement.size
    }
    pub fn require_memory_type_bits(&self) -> uint32_t {
        self.requirement.memory_type_bits
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_buffer(self.handle, None);
        }
    }
}
