use crate::adapter::AdapterType;
use crate::descriptions::AdapterDesc;
use crate::factory::Factory;
use crate::factory::FactoryType;
use crate::output::Output;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::IDXGIAdapter;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, SUCCEEDED, S_OK};
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync)]
#[repr(transparent)]
/// Represents a display sub-system (including one or more GPUs, DACs, and
/// video memory).
pub struct Adapter {
    ptr: ComPtr<IDXGIAdapter>,
}

impl Adapter {
    /// Create an adapter interface that represents a software adapter.
    ///
    /// A software adapter is a DLL that implements the entirety of a device
    /// driver interface, plus emulation, if necessary, of kernel-mode graphics
    /// components for Windows. Details on implementing a software adapter can
    /// be found in the Windows Vista Driver Development Kit. This is a very
    /// complex development task, and is not recommended for general readers.
    pub unsafe fn create_software_adapter(
        factory: &Factory,
        module: HMODULE,
    ) -> Result<Adapter, Error> {
        let mut ptr = std::ptr::null_mut();
        let hr = (*factory.get_raw()).CreateSoftwareAdapter(module, &mut ptr);
        if !SUCCEEDED(hr) {
            return Err(hr.into());
        }

        Ok(Adapter::from_raw(ptr))
    }
}

pub unsafe trait IAdapter {
    /// Gets a description of the adapter (or video card).
    fn desc(&self) -> AdapterDesc {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.raw_adp().GetDesc(&mut desc);
            assert!(SUCCEEDED(hr));
            desc.into()
        }
    }

    /// Checks if an interface is supported by the adapter, e.g. ID3D10Device::uuidof(). Returns
    /// the version number of the usermode driver if it is, None otherwise.
    fn check_interface_support(&self, guid: &GUID) -> Option<i64> {
        unsafe {
            let mut version = std::mem::zeroed();
            let hr = self.raw_adp().CheckInterfaceSupport(guid, &mut version);
            if hr == S_OK {
                Some(*version.QuadPart())
            } else {
                None
            }
        }
    }

    /// Create an iterator that enumerates over the outputs associated with
    /// this adapter.
    fn outputs(&self) -> OutputIter {
        OutputIter {
            adapter: unsafe { self.raw_adp() },
            output: 0,
        }
    }

    /// Get the DXGI Factory associated with this adapter.
    fn factory<F: FactoryType>(&self) -> Option<F>
    where
        Self: Sized,
    {
        imp_factory(self)
    }

    unsafe fn raw_adp(&self) -> &IDXGIAdapter;
}

impl dyn IAdapter + '_ {
    pub fn factory<F: FactoryType>(&self) -> Option<F> {
        imp_factory(self)
    }
}

fn imp_factory<F: FactoryType>(adapter: &dyn IAdapter) -> Option<F> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        let hr = adapter
            .raw_adp()
            .GetParent(&F::Interface::uuidof(), &mut ptr);
        if SUCCEEDED(hr) {
            Some(F::from_raw(ptr as _))
        } else {
            None
        }
    }
}

unsafe impl AdapterType for Adapter {}

unsafe impl IAdapter for Adapter {
    unsafe fn raw_adp(&self) -> &IDXGIAdapter {
        &self.ptr
    }
}

impl std::fmt::Debug for Adapter {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Adapter")
            .field("desc", &self.desc())
            .finish()
    }
}

#[derive(Copy, Clone)]
/// Iterator over the outputs associated with an adapter.
pub struct OutputIter<'a> {
    adapter: &'a IDXGIAdapter,
    output: u32,
}

impl<'a> Iterator for OutputIter<'a> {
    type Item = Output;

    #[inline]
    fn next(&mut self) -> Option<Output> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
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
