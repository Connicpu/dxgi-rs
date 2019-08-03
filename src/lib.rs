//! TODO: Add documentation

#![cfg(windows)]
//#![warn(missing_docs)]

extern crate checked_enum;
extern crate com_wrapper;
extern crate math2d;
extern crate winapi;
extern crate wio;

#[macro_use]
mod helpers;

pub mod adapter;
pub mod descriptions;
pub mod device;
pub mod device_subobject;
pub mod enums;
pub mod factory;
pub mod features;
pub mod output;
pub mod resource;
pub mod surface;
pub mod swap_chain;
