#![cfg(windows)]

#[macro_use]
extern crate auto_enum;

extern crate checked_enum;
extern crate winapi;
extern crate wio;

#[doc(no_inline)]
pub use enums::{
    AlphaMode, Format, ModeRotation, ModeScaling, ModeScanlineOrder, PresentFlags, Scaling,
    SwapChainFlags, SwapEffect, UsageFlags,
};

#[doc(inline)]
pub use adapter::Adapter;
#[doc(inline)]
pub use device::Device;
#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use factory::Factory;
#[doc(inline)]
pub use output::Output;
#[doc(inline)]
pub use surface::Surface;
#[doc(inline)]
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
