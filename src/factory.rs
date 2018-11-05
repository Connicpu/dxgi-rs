use adapter::Adapter;
use enums::WindowAssociationFlags;
use error::Error;

use std::ptr;

use winapi::shared::dxgi::{CreateDXGIFactory1, IDXGIAdapter1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, SUCCEEDED, S_OK};
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The root type of DXGI, which is used for
pub struct Factory {
    ptr: ComPtr<IDXGIFactory2>,
}

impl Factory {
    #[inline]
    /// Creates a new DXGI Factory.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Note**
    /// This method fails if your app's `DllMain` function calls it. For more
    /// info about how DXGI responds from `DllMain`, see
    /// [DXGI Responses from DllMain][1]
    ///
    /// </div>
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/desktop/direct3ddxgi/d3d10-graphics-programming-guide-dxgi#dxgi-responses-from-dllmain
    pub fn new() -> Result<Factory, Error> {
        unsafe {
            let mut ptr = ptr::null_mut();

            let hr = CreateDXGIFactory1(&IDXGIFactory2::uuidof(), &mut ptr);
            Error::map_if(hr, || Factory {
                ptr: ComPtr::from_raw(ptr as *mut _),
            })
        }
    }

    #[inline]
    /// Iterates over all of the adapters (video cards). The first adapter
    /// returned will be the adapter associated with the output on which the
    /// primary desktop is displayed.
    pub fn adapters(&self) -> AdapterIter {
        AdapterIter {
            factory: &self.ptr,
            adapter: 0,
        }
    }

    #[inline]
    /// Gets the `HWND` associated with this DXGI Factory.
    pub fn get_window_association(&self) -> Result<HWND, Error> {
        unsafe {
            let mut hwnd = ptr::null_mut();
            let hr = self.ptr.GetWindowAssociation(&mut hwnd);
            Error::map(hr, hwnd)
        }
    }

    #[inline]
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

    #[inline]
    /// Create an adapter interface that represents a software adapter.
    ///
    /// A software adapter is a DLL that implements the entirety of a device
    /// driver interface, plus emulation, if necessary, of kernel-mode graphics
    /// components for Windows. Details on implementing a software adapter can
    /// be found in the Windows Vista Driver Development Kit. This is a very
    /// complex development task, and is not recommended for general readers.
    pub unsafe fn create_software_adapter(&self, module: HMODULE) -> Result<Adapter, Error> {
        let mut ptr = ptr::null_mut();
        let hr = self.ptr.CreateSoftwareAdapter(module, &mut ptr);
        if !SUCCEEDED(hr) {
            return Err(hr.into());
        }

        let ptr = ComPtr::from_raw(ptr).cast::<IDXGIAdapter1>()?;
        Ok(Adapter::from_raw(ptr.into_raw()))
    }
}

#[derive(Copy, Clone)]
/// An iterator over the graphics adapters on the computer.
pub struct AdapterIter<'a> {
    factory: &'a IDXGIFactory2,
    adapter: u32,
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Adapter;

    fn next(&mut self) -> Option<Adapter> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.factory.EnumAdapters1(self.adapter, &mut ptr);
            self.adapter += 1;

            match result {
                S_OK => Some(Adapter::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!("{} should not be returned from EnumAdapters1", result),
            }
        }
    }
}
