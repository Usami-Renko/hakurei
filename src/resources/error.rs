
use std::fmt;
use std::error::Error;

/// possible error may occur during the creation of vk::Framebuffer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FramebufferError {

    FramebufferCreationError,
}

impl Error for FramebufferError {}
impl fmt::Display for FramebufferError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | FramebufferError::FramebufferCreationError => "Failed to create Framebuffer Object.",
        };
        write!(f, "{}", description)
    }
}

/// possible error may occur during the use of vk::CommandPool and vk::CommandBuffer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandError {

    QueueFamilyUnavailable,
    PoolCreationError,
    BufferAllocateError,
    RecordBeginError,
    RecordEndError,
}

impl Error for CommandError {}
impl fmt::Display for CommandError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | CommandError::QueueFamilyUnavailable => "Graphics Queue Family is not available.",
            | CommandError::PoolCreationError      => "Failed to create Command Pool.",
            | CommandError::BufferAllocateError    => "Failed to allocate Command Buffer.",
            | CommandError::RecordBeginError       => "Failed to begin Command Buffer recording.",
            | CommandError::RecordEndError         => "Failed to end Command Buffer recording.",
        };
        write!(f, "{}", description)
    }
}
