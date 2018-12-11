use crate::adapter::AdapterType;
use crate::enums::GpuPreference;
use crate::error::Error;

use std::marker::PhantomData;

use winapi::shared::dxgi1_6::IDXGIFactory6;
use winapi::shared::winerror::{DXGI_ERROR_NOT_FOUND, S_OK};
use winapi::Interface;
use wio::com::ComPtr;

pub use self::iter::AdapterIterByPreference;

mod iter;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory6 {
    ptr: ComPtr<IDXGIFactory6>,
}

impl Factory6 {
    pub fn adapters_by_preference<A: AdapterType>(
        &self,
        preference: GpuPreference,
    ) -> AdapterIterByPreference<A> {
        AdapterIterByPreference {
            factory: self,
            adapter: 0,
            preference,
            _marker: PhantomData,
        }
    }

    /// Attempt to get the Nth adapter, sorted by the given preference.
    pub fn enum_adapter_by_preference<A: AdapterType>(
        &self,
        n: u32,
        preference: GpuPreference,
    ) -> Option<Result<A, Error>> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.EnumAdapterByGpuPreference(
                n,
                preference as u32,
                &A::Interface::uuidof(),
                &mut ptr,
            );

            match hr {
                S_OK => Some(Ok(A::from_raw(ptr as _))),
                DXGI_ERROR_NOT_FOUND => None,
                error => Some(Err(error.into())),
            }
        }
    }
}

impl super::FactoryType for Factory6 {}

impl std::ops::Deref for Factory6 {
    type Target = super::Factory5;
    fn deref(&self) -> &Self::Target {
        unsafe { crate::helpers::deref_com_wrapper(self) }
    }
}

impl std::ops::DerefMut for Factory6 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { crate::helpers::deref_com_wrapper_mut(self) }
    }
}


