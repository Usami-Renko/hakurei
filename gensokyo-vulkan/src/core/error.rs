
use std::fmt;
use std::error::Error;

use gsma::impl_from_err;

use crate::command::CommandError;

/// possible error may occur during the creation of vk::Instance.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InstanceError {

    EntryCreationError,
    ValidationLayerNotSupportError,
    InstanceCreationError,
    LayerPropertiesEnumerateError,
}

impl Error for InstanceError {}
impl fmt::Display for InstanceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | InstanceError::EntryCreationError             => "Failed to create Entry Object.",
            | InstanceError::ValidationLayerNotSupportError => "Validation Layer is not support.",
            | InstanceError::InstanceCreationError          => "Failed to create Instance Object.",
            | InstanceError::LayerPropertiesEnumerateError  => "Failed to enumerate Instance Layer Properties.",
        };

        write!(f, "{}", description)
    }
}


/// possible error may occur during the initialization of Validation Layer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValidationError {

    DebugReportCallbackCreationError,
    DebugUtilsCallbackCreationEror,
}

impl Error for ValidationError {}
impl fmt::Display for ValidationError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ValidationError::DebugReportCallbackCreationError => "Failed to create DebugReport Callback Object.",
            | ValidationError::DebugUtilsCallbackCreationEror   => "Failed to create DebugUtils Callback Object.",
        };

        write!(f, "{}", description)
    }
}


/// possible error may occur during the creation of vk::PhysicalDevice.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PhysicalDeviceError {

    NoSuitableDeviceError,
    EnumerateDeviceError,
    GraphicsQueueNotSupportError,
    PresentQueueNotSupportError,
    TransferQueueNotSupportError,
    EnumerateExtensionsError,
    FormatUsageNotSupport(PhysicalFormatUsage),
}

impl Error for PhysicalDeviceError {}
impl fmt::Display for PhysicalDeviceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | PhysicalDeviceError::NoSuitableDeviceError        => "No Physical Device suitable for requirements.",
            | PhysicalDeviceError::EnumerateDeviceError         => "Failed to enumerate Physical Devices.",
            | PhysicalDeviceError::GraphicsQueueNotSupportError => "Physical device does not support graphics requirement.",
            | PhysicalDeviceError::PresentQueueNotSupportError  => "Physical device does not support present requirement.",
            | PhysicalDeviceError::TransferQueueNotSupportError => "Physical device does not support transfer requirement",
            | PhysicalDeviceError::EnumerateExtensionsError     => "Failed to enumerate Device Extensions.",
            | PhysicalDeviceError::FormatUsageNotSupport(usage) => return write!(f, "Unable to find support format for {}.", usage),
        };

        write!(f, "{}", description)
    }
}

/// The possible specific required usage of vk::Format in Vulkan.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PhysicalFormatUsage {
    DepthStencil,
}

impl fmt::Display for PhysicalFormatUsage {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | PhysicalFormatUsage::DepthStencil => "Depth or Stencil Buffer",
        };

        write!(f, "{}", description)
    }
}

/// possible error may occur during the creation of vk::Surface.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SurfaceError {

    SurfaceCreationError,
    QueryCapabilitiesError,
    QueryFormatsError,
    QueryPresentModeError,
}

impl Error for SurfaceError {}
impl fmt::Display for SurfaceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | SurfaceError::SurfaceCreationError   => "Failed to create Surface.",
            | SurfaceError::QueryCapabilitiesError => "Failed to query surface capabilities.",
            | SurfaceError::QueryFormatsError      => "Failed to query surface formats.",
            | SurfaceError::QueryPresentModeError  => "Failed to query surface present mode.",
        };

        write!(f, "{}", description)
    }
}


/// possible error may occur during the creation of vk::Device.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogicalDeviceError {

    DeviceCreationError,
    WaitIdleError,
    QueueOpsUnsupport,
    QueueCountNotEnough,
    Command(CommandError),
}

impl Error for LogicalDeviceError {}
impl fmt::Display for LogicalDeviceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            | LogicalDeviceError::DeviceCreationError  => write!(f, "Failed to create Logical Device."),
            | LogicalDeviceError::WaitIdleError        => write!(f, "Device failed to wait idle."),
            | LogicalDeviceError::QueueOpsUnsupport    => write!(f, "Not all the operations is support for Device Queues."),
            | LogicalDeviceError::QueueCountNotEnough  => write!(f, "No enough queue available on this Device."),
            | LogicalDeviceError::Command(e)           => write!(f, "{}", e.to_string()),
        }
    }
}

impl_from_err!(Command(CommandError) -> LogicalDeviceError);
