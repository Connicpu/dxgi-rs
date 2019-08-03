use crate::adapter::adapter1::Adapter1;
use crate::factory::{FactoryType, IFactory};

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIFactory, IDXGIFactory1};
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, S_OK};
use wio::com::ComPtr;

pub use self::iter::AdapterIter1;

mod iter;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The Factory1 interface allows iterating adapters with the Adapter1 type instead of just Adapter.
pub struct Factory1 {
    ptr: ComPtr<IDXGIFactory1>,
}

pub unsafe trait IFactory1: IFactory {
    /// Informs an application of the possible need to re-enumerate adapters.
    fn is_current(&self) -> bool {
        unsafe { self.raw_f1().IsCurrent() != 0 }
    }

    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    fn adapters1(&self) -> AdapterIter1<Self>
    where
        Self: Sized,
    {
        AdapterIter1 {
            factory: self,
            adapter: 0,
        }
    }

    /// Attempt to get the Nth adapter
    fn enum_adapter1(&self, n: u32) -> Option<Adapter1> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_f1().EnumAdapters1(n, &mut ptr);
            match hr {
                S_OK => Some(Adapter1::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!("{} should not be returned from EnumAdapters1", result),
            }
        }
    }

    unsafe fn raw_f1(&self) -> &IDXGIFactory1;
}

impl dyn IFactory1 + '_ {
    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    pub fn adapters1_dyn(&self) -> AdapterIter1<Self> {
        AdapterIter1 {
            factory: self,
            adapter: 0,
        }
    }
}

unsafe impl IFactory for Factory1 {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl IFactory1 for Factory1 {
    unsafe fn raw_f1(&self) -> &IDXGIFactory1 {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory1 {}
