use crate::adapter::AdapterType;
use crate::adapter::IAdapter;
use crate::descriptions::AdapterDesc1;

use std::fmt;
use std::mem::MaybeUninit;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIAdapter1};
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync)]
#[repr(transparent)]
/// Represents a display sub-system (including one or more GPUs, DACs, and
/// video memory).
pub struct Adapter1 {
    ptr: ComPtr<IDXGIAdapter1>,
}

pub unsafe trait IAdapter1: IAdapter {
    /// Gets a description of the adapter (or video card).
    fn desc1(&self) -> AdapterDesc1 {
        unsafe {
            let mut desc = MaybeUninit::uninit();
            let hr = self.raw_adp1().GetDesc1(desc.as_mut_ptr());
            assert!(SUCCEEDED(hr));
            desc.assume_init().into()
        }
    }

    unsafe fn raw_adp1(&self) -> &IDXGIAdapter1;
}

unsafe impl IAdapter for Adapter1 {
    unsafe fn raw_adp(&self) -> &IDXGIAdapter {
        &self.ptr
    }
}

unsafe impl IAdapter1 for Adapter1 {
    unsafe fn raw_adp1(&self) -> &IDXGIAdapter1 {
        &self.ptr
    }
}

unsafe impl AdapterType for Adapter1 {}

impl fmt::Debug for Adapter1 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Adapter")
            .field("desc", &self.desc1())
            .finish()
    }
}
