use adapter::Adapter;
use adapter::AdapterType;
use descriptions::AdapterDesc1;
use factory::FactoryType;
use helpers::deref_com_wrapper;

use std::fmt;
use std::mem;

use winapi::shared::dxgi::IDXGIAdapter1;
use winapi::shared::winerror::SUCCEEDED;
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync)]
#[repr(transparent)]
/// Represents a display sub-system (including one or more GPUs, DACs, and
/// video memory).
pub struct Adapter1 {
    ptr: ComPtr<IDXGIAdapter1>,
}

impl Adapter1 {
    /// Gets a description of the adapter (or video card).
    #[inline]
    pub fn desc(&self) -> AdapterDesc1 {
        unsafe {
            let mut desc = mem::uninitialized();
            let hr = self.ptr.GetDesc1(&mut desc);
            assert!(SUCCEEDED(hr));
            desc.into()
        }
    }

    /// Get the DXGI Factory associated with this adapter.
    #[inline]
    pub fn factory<F: FactoryType>(&self) -> Option<F> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetParent(&F::Interface::uuidof(), &mut ptr);
            if SUCCEEDED(hr) {
                Some(F::from_raw(ptr as _))
            } else {
                None
            }
        }
    }
}

impl std::ops::Deref for Adapter1 {
    type Target = Adapter;
    fn deref(&self) -> &Adapter {
        unsafe { deref_com_wrapper(self) }
    }
}

impl AdapterType for Adapter1 {}

impl fmt::Debug for Adapter1 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Adapter")
            .field("desc", &self.desc())
            .finish()
    }
}
