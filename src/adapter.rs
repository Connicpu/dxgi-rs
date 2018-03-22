use error::Error;
use output::Output;

use std::fmt;
use std::ptr;
use std::mem;
use std::ffi::OsString;

use winapi::shared::dxgi::{DXGI_ADAPTER_DESC1, IDXGIAdapter1};
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, S_OK};
use winapi::um::winnt::LUID;
use wio::com::ComPtr;
use wio::wide::FromWide;

pub struct Adapter {
    ptr: ComPtr<IDXGIAdapter1>,
}

impl Adapter {
    pub unsafe fn from_raw(ptr: *mut IDXGIAdapter1) -> Adapter {
        Adapter {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDXGIAdapter1 {
        self.ptr.as_raw()
    }

    pub fn get_desc(&self) -> AdapterDesc {
        unsafe {
            let mut desc = mem::uninitialized();
            let result = self.ptr.GetDesc1(&mut desc);
            assert!(
                result >= 0,
                "The only failure case of GetDesc1 is pDesc being null"
            );

            AdapterDesc { desc }
        }
    }

    pub fn outputs(&self) -> OutputIter {
        OutputIter {
            adapter: self,
            output: 0,
        }
    }
}

impl fmt::Debug for Adapter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Adapter")
            .field("desc", &self.get_desc())
            .finish()
    }
}

pub struct AdapterDesc {
    desc: DXGI_ADAPTER_DESC1,
}

impl AdapterDesc {
    pub fn description(&self) -> String {
        let len = self.desc
            .Description
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(128);
        let ostr = OsString::from_wide(&self.desc.Description[..len]);
        ostr.to_string_lossy().into_owned()
    }

    pub fn vendor_id(&self) -> u32 {
        self.desc.VendorId
    }

    pub fn device_id(&self) -> u32 {
        self.desc.DeviceId
    }

    pub fn sub_sys_id(&self) -> u32 {
        self.desc.SubSysId
    }

    pub fn revision(&self) -> u32 {
        self.desc.Revision
    }

    pub fn dedicated_video_memory(&self) -> usize {
        self.desc.DedicatedVideoMemory
    }

    pub fn dedicated_system_memory(&self) -> usize {
        self.desc.DedicatedSystemMemory
    }

    pub fn shared_system_memory(&self) -> usize {
        self.desc.SharedSystemMemory
    }

    pub fn adapter_luid(&self) -> LUID {
        self.desc.AdapterLuid
    }

    pub fn flags(&self) -> u32 {
        self.desc.Flags
    }
}

impl fmt::Debug for AdapterDesc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AdapterDesc")
            .field("description", &self.description())
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .field("sub_sys_id", &self.sub_sys_id())
            .field("revision", &self.revision())
            .field("dedicated_video_memory", &self.dedicated_video_memory())
            .field("dedicated_system_memory", &self.dedicated_system_memory())
            .field("shared_system_memory", &self.shared_system_memory())
            .field("adapter_luid", &unsafe {
                mem::transmute::<_, i64>(self.adapter_luid())
            })
            .field("flags", &self.flags())
            .finish()
    }
}

pub struct OutputIter<'a> {
    adapter: &'a Adapter,
    output: u32,
}

impl<'a> Iterator for OutputIter<'a> {
    type Item = Output;
    fn next(&mut self) -> Option<Output> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.adapter.ptr.EnumOutputs(self.output, &mut ptr);
            self.output += 1;

            match result {
                S_OK => Some(Output::from_raw(ptr)),
                DXGI_ERROR_NOT_FOUND => None,
                result => unreachable!(
                    "`{}` should not be returned from EnumAdapters1",
                    Error(result)
                ),
            }
        }
    }
}
