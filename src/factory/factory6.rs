use crate::adapter::AdapterType;
use crate::enums::GpuPreference;
use crate::factory::{
    FactoryType, IFactory, IFactory1, IFactory2, IFactory3, IFactory4, IFactory5,
};

use std::marker::PhantomData;

use com_wrapper::ComWrapper;
use dcommon::error::Error;
use winapi::shared::dxgi::{IDXGIFactory, IDXGIFactory1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::dxgi1_3::IDXGIFactory3;
use winapi::shared::dxgi1_4::IDXGIFactory4;
use winapi::shared::dxgi1_5::IDXGIFactory5;
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

pub unsafe trait IFactory6: IFactory5 {
    /// Iterate the adapters ordered by a GpuPreference.
    fn adapters_by_preference<A: AdapterType>(
        &self,
        preference: GpuPreference,
    ) -> AdapterIterByPreference<A>
    where
        Self: Sized,
    {
        AdapterIterByPreference {
            factory: self,
            adapter: 0,
            preference,
            _marker: PhantomData,
        }
    }

    /// Attempt to get the Nth adapter, sorted by the given preference.
    fn enum_adapter_by_preference<A: AdapterType>(
        &self,
        n: u32,
        preference: GpuPreference,
    ) -> Option<Result<A, Error>>
    where
        Self: Sized,
    {
        imp_enum_adapter_by_preference(self, n, preference)
    }

    unsafe fn raw_f6(&self) -> &IDXGIFactory6;
}

impl dyn IFactory6 + '_ {
    /// Iterate the adapters ordered by a GpuPreference.
    pub fn adapters_by_preference_dyn<A: AdapterType>(
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
    pub fn enum_adapter_by_preference_dyn<A: AdapterType>(
        &self,
        n: u32,
        preference: GpuPreference,
    ) -> Option<Result<A, Error>> {
        imp_enum_adapter_by_preference(self, n, preference)
    }
}

fn imp_enum_adapter_by_preference<A: AdapterType>(
    f: &dyn IFactory6,
    n: u32,
    preference: GpuPreference,
) -> Option<Result<A, Error>> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        let hr = f.raw_f6().EnumAdapterByGpuPreference(
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

unsafe impl IFactory for Factory6 {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl IFactory1 for Factory6 {
    unsafe fn raw_f1(&self) -> &IDXGIFactory1 {
        &self.ptr
    }
}

unsafe impl IFactory2 for Factory6 {
    unsafe fn raw_f2(&self) -> &IDXGIFactory2 {
        &self.ptr
    }
}

unsafe impl IFactory3 for Factory6 {
    unsafe fn raw_f3(&self) -> &IDXGIFactory3 {
        &self.ptr
    }
}

unsafe impl IFactory4 for Factory6 {
    unsafe fn raw_f4(&self) -> &IDXGIFactory4 {
        &self.ptr
    }
}

unsafe impl IFactory5 for Factory6 {
    unsafe fn raw_f5(&self) -> &IDXGIFactory5 {
        &self.ptr
    }
}

unsafe impl IFactory6 for Factory6 {
    unsafe fn raw_f6(&self) -> &IDXGIFactory6 {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory6 {}
