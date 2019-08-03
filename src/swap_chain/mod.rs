use crate::output::Output;

use com_wrapper::ComWrapper;

pub use self::resize_buffers::ResizeBuffers;
pub use self::swap_chain::{ISwapChain, SwapChain};
pub use self::swap_chain1::{ISwapChain1, SwapChain1};

pub mod resize_buffers;
pub mod swap_chain;
pub mod swap_chain1;

pub unsafe trait CoreWindowType: ComWrapper {}

/// This should be implemented for e.g. d3d11::Texture2d
pub unsafe trait BackbufferTexture: ComWrapper {}

pub enum FullscreenState {
    Windowed,
    Fullscreen(Option<Output>),
}
