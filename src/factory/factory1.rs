use crate::adapter::adapter1::Adapter1;
use crate::factory::FactoryType;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIFactory1;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, S_OK};
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The Factory1 interface allows iterating adapters with the Adapter1 type instead of just Adapter.
pub struct Factory1 {
    ptr: ComPtr<IDXGIFactory1>,
}

impl Factory1 {
    /// Informs an application of the possible need to re-enumerate adapters.
    pub fn is_current(&self) -> bool {
        unsafe { self.ptr.IsCurrent() != 0 }
    }

    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    pub fn adapters(&self) -> AdapterIter1 {
        AdapterIter1 {
            factory: self,
            adapter: 0,
        }
    }

    /// Attempt to get the Nth adapter
    pub fn adapter(&self, n: u32) -> Option<Adapter1> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.EnumAdapters1(n, &mut ptr);
            match hr {
                S_OK => Some(Adapter1::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!("{} should not be returned from EnumAdapters1", result),
            }
        }
    }
}

impl FactoryType for Factory1 {}

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter1<'a> {
    factory: &'a Factory1,
    adapter: u32,
}

impl<'a> Iterator for AdapterIter1<'a> {
    type Item = Adapter1;

    fn next(&mut self) -> Option<Adapter1> {
        let result = self.factory.adapter(self.adapter);
        self.adapter += 1;
        result
    }
}