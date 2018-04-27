#![cfg(windows)]

extern crate boolinator;
extern crate num;
extern crate winapi;
extern crate wio;

pub use adapter::Adapter;
pub use device::Device;
pub use error::Error;
pub use factory::Factory;
pub use flags::{AlphaMode, Format, ModeRotation, ModeScaling, ModeScanlineOrder, PresentFlags,
                Scaling, SwapChainFlags, SwapEffect, UsageFlags};
pub use output::Output;
pub use surface::Surface;
pub use swap_chain::SwapChain;

pub mod adapter;
pub mod device;
pub mod error;
pub mod factory;
pub mod flags;
pub mod output;
pub mod surface;
pub mod swap_chain;
