
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use resources::buffer::BufferSubItem;
use resources::memory::{ HaMemoryAbstract, MemoryDataUploadable, MemoryMapable };
use resources::memory::{ HaMemoryType, UploadStagingResource };
use resources::memory::{ MemoryRange, MemoryMapStatus };
use resources::allocator::BufferAllocateInfos;
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;

use std::ptr;

pub struct HaStagingMemory {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    mem_type   : Option<vk::MemoryType>,

    map_status : MemoryMapStatus,
}

impl MemoryMapable for HaStagingMemory {}

impl HaMemoryAbstract for HaStagingMemory {

    fn handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn flag(&self) -> vk::MemoryPropertyFlags {
        self.mem_type.as_ref()
            .and_then(|m| Some(m.property_flags))
            .unwrap_or(vk::MemoryPropertyFlags::empty())
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::StagingMemory
    }

    fn default_flag() -> vk::MemoryPropertyFlags {
        HaMemoryType::StagingMemory.property_flags()
    }

    fn allocate(device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<HaStagingMemory, MemoryError> {

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: mem_type_index as uint32_t,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };

        let memory = HaStagingMemory {
            handle,
            _size: size,
            mem_type,
            map_status: MemoryMapStatus::from_unmap(),
        };
        Ok(memory)
    }
}

impl HaStagingMemory {

    fn enable_map(&mut self, device: &HaDevice, is_enable: bool) -> Result<(), MemoryError> {

        if is_enable {
            if !self.map_status.is_map {
                let ptr = self.map_range(device, None)?;
                self.map_status.set_map(ptr);
            }
        } else {
            if self.map_status.is_map {
                self.unmap(device);
                self.map_status.invaild_map();
            }
        }

        Ok(())
    }
}


impl MemoryDataUploadable for HaStagingMemory {

    fn prepare_data_transfer(&mut self, _: &HaPhyDevice, device: &HaDevice, _: &Option<BufferAllocateInfos>) -> Result<Option<UploadStagingResource>, MemoryError> {

        self.enable_map(device, true)?;
        Ok(None)
    }

    fn map_memory_ptr(&mut self, _: &mut Option<UploadStagingResource>, item: &BufferSubItem, offset: vk::DeviceSize) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let ptr = unsafe {
            self.map_status.data_ptr.offset(offset as isize)
        };

        let writer = MemoryWritePtr::new(ptr, item.size);
        let range = MemoryRange { offset, size: item.size };
        Ok((writer, range))
    }

    fn terminate_transfer(&mut self, device: &HaDevice, _: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        if !self.is_coherent_memroy() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.flush_ranges(device, ranges_to_flush)?;
        }

        self.enable_map(device, false)?;

        Ok(())
    }
}