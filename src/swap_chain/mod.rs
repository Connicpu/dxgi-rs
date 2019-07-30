use crate::output::Output;

use com_wrapper::ComWrapper;

pub use self::resize_buffers::ResizeBuffers;
pub use self::swap_chain::SwapChain;
pub use self::swap_chain1::SwapChain1;

pub mod resize_buffers;
pub mod swap_chain;
pub mod swap_chain1;

pub trait CoreWindowType: ComWrapper {}

/// This should be implemented for e.g. d3d11::Texture2d
pub trait BackbufferTexture: ComWrapper {}

pub enum FullscreenState {
    Windowed,
    Fullscreen(Option<Output>),
}
