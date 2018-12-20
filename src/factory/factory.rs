use crate::adapter::Adapter;
use crate::enums::WindowAssociationFlags;
use dcommon::error::Error;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIFactory;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, S_OK};
use wio::com::ComPtr;

pub use self::iter::AdapterIter;

mod iter;

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
            factory: self,
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

    /// Attempt to get the Nth adapter
    pub fn enum_adapter(&self, n: u32) -> Option<Adapter> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.EnumAdapters(n, &mut ptr);
            match hr {
                S_OK => Some(Adapter::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!("{} should not be returned from EnumAdapters1", result),
            }
        }
    }
}

impl super::FactoryType for Factory {}


