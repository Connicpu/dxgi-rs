//! TODO: Add documentation

#![cfg(windows)]
//#![warn(missing_docs)]

#[macro_use]
extern crate auto_enum;

#[macro_use]
extern crate derive_com_wrapper;

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
pub mod enums;
pub mod error;
pub mod factory;
pub mod output;
pub mod resource;
pub mod surface;
pub mod swap_chain;
pub mod device_subobject;
