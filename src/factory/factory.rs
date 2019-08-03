use crate::adapter::Adapter;
use crate::enums::WindowAssociationFlags;
use crate::factory::FactoryType;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
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

pub unsafe trait IFactory {
    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    fn adapters(&self) -> AdapterIter<Self>
    where
        Self: Sized,
    {
        AdapterIter {
            factory: self,
            adapter: 0,
        }
    }

    /// Gets the `HWND` associated with this DXGI Factory.
    fn window_association(&self) -> Result<HWND, Error> {
        unsafe {
            let mut hwnd = std::ptr::null_mut();
            let hr = self.raw_f().GetWindowAssociation(&mut hwnd);
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
    unsafe fn make_window_association(
        &self,
        hwnd: HWND,
        flags: WindowAssociationFlags,
    ) -> Result<(), Error> {
        let hr = self.raw_f().MakeWindowAssociation(hwnd, flags.0);
        Error::map(hr, ())
    }

    /// Attempt to get the Nth adapter
    fn enum_adapter(&self, n: u32) -> Option<Adapter> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_f().EnumAdapters(n, &mut ptr);
            match hr {
                S_OK => Some(Adapter::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!(
                    "{:?} should not be returned from EnumAdapters1",
                    Error::from(result)
                ),
            }
        }
    }

    unsafe fn raw_f(&self) -> &IDXGIFactory;
}

impl dyn IFactory + '_ {
    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    pub fn adapters_dyn(&self) -> AdapterIter<Self> {
        AdapterIter {
            factory: self,
            adapter: 0,
        }
    }
}

unsafe impl IFactory for Factory {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory {}
