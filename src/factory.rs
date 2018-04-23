use adapter::Adapter;
use device::Device;
use error::Error;
use swap_chain::SwapChainHwndBuilder;

use std::ptr;

use winapi::Interface;
use winapi::shared::dxgi::{CreateDXGIFactory1, IDXGIAdapter1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::minwindef::{HMODULE, UINT};
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, SUCCEEDED, S_OK};
use wio::com::ComPtr;

#[derive(Clone, PartialEq)]
pub struct Factory {
    ptr: ComPtr<IDXGIFactory2>,
}

impl Factory {
    #[inline]
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
    pub unsafe fn from_raw(ptr: *mut IDXGIFactory2) -> Factory {
        Factory {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut IDXGIFactory2 {
        self.ptr.as_raw()
    }

    #[inline]
    pub fn adapters(&self) -> AdapterIter {
        AdapterIter {
            factory: &self.ptr,
            adapter: 0,
        }
    }

    #[inline]
    pub fn get_window_association(&self) -> Result<HWND, Error> {
        unsafe {
            let mut hwnd = ptr::null_mut();
            let hr = self.ptr.GetWindowAssociation(&mut hwnd);
            Error::map(hr, hwnd)
        }
    }

    #[inline]
    pub unsafe fn make_window_association(&self, hwnd: HWND, flags: UINT) -> Result<(), Error> {
        let hr = self.ptr.MakeWindowAssociation(hwnd, flags);
        Error::map(hr, ())
    }

    #[inline]
    pub unsafe fn create_software_adapter(&self, module: HMODULE) -> Result<Adapter, Error> {
        let mut ptr = ptr::null_mut();
        let hr = self.ptr.CreateSoftwareAdapter(module, &mut ptr);
        if !SUCCEEDED(hr) {
            return Err(hr.into());
        }

        let ptr = ComPtr::from_raw(ptr).cast::<IDXGIAdapter1>()?;
        Ok(Adapter::from_raw(ptr.into_raw()))
    }

    #[inline]
    pub fn create_swapchain_for_hwnd<'a>(&'a self, device: &'a Device) -> SwapChainHwndBuilder<'a> {
        SwapChainHwndBuilder::create(self, device)
    }
}

unsafe impl Send for Factory {}
unsafe impl Sync for Factory {}

#[derive(Copy, Clone)]
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
