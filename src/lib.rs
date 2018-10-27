#![cfg(windows)]

#[macro_use]
extern crate auto_enum;

extern crate checked_enum;
extern crate winapi;
extern crate wio;

pub use adapter::Adapter;
pub use device::Device;
pub use enums::{
    AlphaMode, Format, ModeRotation, ModeScaling, ModeScanlineOrder, PresentFlags, Scaling,
    SwapChainFlags, SwapEffect, UsageFlags,
};
pub use error::Error;
pub use factory::Factory;
pub use output::Output;
pub use surface::Surface;
pub use swap_chain::SwapChain;

pub mod adapter;
pub mod device;
pub mod enums;
pub mod error;
pub mod factory;
pub mod output;
pub mod ratio;
pub mod surface;
pub mod swap_chain;
