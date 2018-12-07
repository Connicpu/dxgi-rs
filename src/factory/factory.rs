use crate::adapter::Adapter;
use crate::enums::WindowAssociationFlags;
use crate::error::Error;
use crate::factory::FactoryType;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIFactory;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, S_OK};
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The Factory interface is required for generating DXGI objects
/// (which handle full screen transitions).
pub struct Factory {
    ptr: ComPtr<IDXGIFactory>,
}

impl Factory {
    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    pub fn adapters(&self) -> AdapterIter {
        AdapterIter {
            factory: &self.ptr,
            adapter: 0,
        }
    }

    /// Gets the `HWND` associated with this DXGI Factory.
    pub fn window_association(&self) -> Result<HWND, Error> {
        unsafe {
            let mut hwnd = ptr::null_mut();
            let hr = self.ptr.GetWindowAssociation(&mut hwnd);
            Error::map(hr, hwnd)
        }
    }

    /// Associates a window handle with this DXGI Factory so that DXGI may
    /// respond to window events to change modes.
    ///
    /// Pass `NONE` for the flags to use the default behaviors, which is
    /// likely the normal behavior a user expects from a game. If your app is
    /// not a game you may want to look into the various flags and their
    /// implications for your program.
    pub unsafe fn make_window_association(
        &self,
        hwnd: HWND,
        flags: WindowAssociationFlags,
    ) -> Result<(), Error> {
        let hr = self.ptr.MakeWindowAssociation(hwnd, flags.0);
        Error::map(hr, ())
    }
}

impl FactoryType for Factory {}

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter<'a> {
    factory: &'a IDXGIFactory,
    adapter: u32,
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Adapter;

    fn next(&mut self) -> Option<Adapter> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.factory.EnumAdapters(self.adapter, &mut ptr);
            self.adapter += 1;

            match result {
                S_OK => Some(Adapter::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!("{} should not be returned from EnumAdapters1", result),
            }
        }
    }
}
