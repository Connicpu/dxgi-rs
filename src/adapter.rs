use enums::AdapterFlags;
use error::Error;
use factory::Factory;
use output::Output;

use std::ffi::OsString;
use std::fmt;
use std::mem;
use std::ptr;

use winapi::shared::dxgi::{IDXGIAdapter1, DXGI_ADAPTER_DESC1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, SUCCEEDED, S_OK};
use winapi::um::winnt::LUID;
use winapi::Interface;
use wio::com::ComPtr;
use wio::wide::FromWide;

#[derive(Clone, PartialEq)]
/// Represents a display sub-system (including one or more GPUs, DACs, and
/// video memory).
pub struct Adapter {
    ptr: ComPtr<IDXGIAdapter1>,
}

impl Adapter {
    /// Gets a description of the adapter (or video card).
    #[inline]
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

    /// Create an iterator that enumerates over the outputs associated with
    /// this adapter.
    #[inline]
    pub fn outputs(&self) -> OutputIter {
        OutputIter {
            adapter: &self.ptr,
            output: 0,
        }
    }

    /// Get the DXGI Factory associated with this adapter.
    #[inline]
    pub fn get_factory(&self) -> Factory {
        unsafe {
            let mut factory = ptr::null_mut();
            let hr = self.ptr.GetParent(&IDXGIFactory2::uuidof(), &mut factory);
            assert!(SUCCEEDED(hr));
            Factory::from_raw(factory as *mut _)
        }
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut IDXGIAdapter1) -> Adapter {
        Adapter {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut IDXGIAdapter1 {
        self.ptr.as_raw()
    }
}

unsafe impl Send for Adapter {}
unsafe impl Sync for Adapter {}

impl fmt::Debug for Adapter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Adapter")
            .field("desc", &self.get_desc())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct AdapterDesc {
    desc: DXGI_ADAPTER_DESC1,
}

impl AdapterDesc {
    #[inline]
    /// A string that contains the adapter description. On feature level 9
    /// graphics hardware, `get_desc` returns `“Software Adapter”` for the
    /// description string.
    pub fn description(&self) -> String {
        let len = self
            .desc
            .Description
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(128);
        let ostr = OsString::from_wide(&self.desc.Description[..len]);
        ostr.to_string_lossy().into_owned()
    }

    #[inline]
    /// The PCI ID of the hardware vendor. On feature level 9 graphics
    /// hardware, `get_desc` returns zeros for the PCI ID of the hardware
    /// vendor.
    pub fn vendor_id(&self) -> u32 {
        self.desc.VendorId
    }

    #[inline]
    /// The PCI ID of the hardware device. On feature level 9 graphics
    /// hardware, `get_desc` returns zeros for the PCI ID of the hardware
    /// device.
    pub fn device_id(&self) -> u32 {
        self.desc.DeviceId
    }

    #[inline]
    /// The PCI ID of the sub system. On feature level 9 graphics hardware,
    /// `get_desc` returns zeros for the PCI ID of the sub system.
    pub fn sub_sys_id(&self) -> u32 {
        self.desc.SubSysId
    }

    #[inline]
    /// The PCI ID of the revision number of the adapter. On feature level 9
    /// graphics hardware, `get_desc` returns zeros for the PCI ID of the
    /// revision number of the adapter.
    pub fn revision(&self) -> u32 {
        self.desc.Revision
    }

    #[inline]
    /// The number of bytes of dedicated video memory that are not shared with
    /// the CPU.
    pub fn dedicated_video_memory(&self) -> usize {
        self.desc.DedicatedVideoMemory
    }

    #[inline]
    /// The number of bytes of dedicated system memory that are not shared with
    /// the CPU. This memory is allocated from available system memory at boot time.
    pub fn dedicated_system_memory(&self) -> usize {
        self.desc.DedicatedSystemMemory
    }

    #[inline]
    /// The number of bytes of shared system memory. This is the maximum value
    /// of system memory that may be consumed by the adapter during operation.
    /// Any incidental memory consumed by the driver as it manages and uses
    /// video memory is additional.
    pub fn shared_system_memory(&self) -> usize {
        self.desc.SharedSystemMemory
    }

    #[inline]
    /// A unique value that identifies the adapter.
    pub fn adapter_luid(&self) -> LUID {
        self.desc.AdapterLuid
    }

    #[inline]
    /// The adapter flags that describe the adapter type. `REMOTE` is reserved.
    pub fn flags(&self) -> AdapterFlags {
        AdapterFlags(self.desc.Flags)
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

#[derive(Copy, Clone)]
/// Iterator over the outputs associated with an adapter.
pub struct OutputIter<'a> {
    adapter: &'a IDXGIAdapter1,
    output: u32,
}

impl<'a> Iterator for OutputIter<'a> {
    type Item = Output;

    #[inline]
    fn next(&mut self) -> Option<Output> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.adapter.EnumOutputs(self.output, &mut ptr);
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
