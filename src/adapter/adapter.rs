use adapter::AdapterType;
use descriptions::AdapterDesc;
use error::Error;
use factory::factory::Factory;
use factory::FactoryType;
use output::Output;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::IDXGIAdapter;
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
    #[inline]
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

    /// Gets a description of the adapter (or video card).
    #[inline]
    pub fn desc(&self) -> AdapterDesc {
        unsafe {
            let mut desc = std::mem::zeroed();
            let hr = self.ptr.GetDesc(&mut desc);
            assert!(SUCCEEDED(hr));
            desc.into()
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
    pub fn factory<F: FactoryType>(&self) -> Option<F> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetParent(&F::Interface::uuidof(), &mut ptr);
            if SUCCEEDED(hr) {
                Some(F::from_raw(ptr as _))
            } else {
                None
            }
        }
    }
}

impl AdapterType for Adapter {}

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
