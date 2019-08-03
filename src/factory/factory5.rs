use crate::factory::{FactoryType, IFactory, IFactory1, IFactory2, IFactory3, IFactory4};
use crate::features::Feature;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIFactory, IDXGIFactory1};
use winapi::shared::dxgi1_2::IDXGIFactory2;
use winapi::shared::dxgi1_3::IDXGIFactory3;
use winapi::shared::dxgi1_4::IDXGIFactory4;
use winapi::shared::dxgi1_5::IDXGIFactory5;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Factory5 {
    ptr: ComPtr<IDXGIFactory5>,
}

pub unsafe trait IFactory5: IFactory4 {
    fn check_feature_support<F: Feature>(&self) -> F::Result
    where
        Self: Sized,
    {
        imp_check_feature_support::<F>(self)
    }

    unsafe fn raw_f5(&self) -> &IDXGIFactory5;
}

impl dyn IFactory5 + '_ {
    pub fn check_feature_support_dyn<F: Feature>(&self) -> F::Result {
        imp_check_feature_support::<F>(self)
    }
}

fn imp_check_feature_support<F: Feature>(f: &dyn IFactory5) -> F::Result {
    unsafe {
        let mut data: F::Structure = std::mem::zeroed();
        let hr = f.raw_f5().CheckFeatureSupport(
            F::FLAG,
            (&mut data) as *mut _ as _,
            std::mem::size_of_val(&data) as u32,
        );
        F::get_result(hr, &data)
    }
}

unsafe impl IFactory for Factory5 {
    unsafe fn raw_f(&self) -> &IDXGIFactory {
        &self.ptr
    }
}

unsafe impl IFactory1 for Factory5 {
    unsafe fn raw_f1(&self) -> &IDXGIFactory1 {
        &self.ptr
    }
}

unsafe impl IFactory2 for Factory5 {
    unsafe fn raw_f2(&self) -> &IDXGIFactory2 {
        &self.ptr
    }
}

unsafe impl IFactory3 for Factory5 {
    unsafe fn raw_f3(&self) -> &IDXGIFactory3 {
        &self.ptr
    }
}

unsafe impl IFactory4 for Factory5 {
    unsafe fn raw_f4(&self) -> &IDXGIFactory4 {
        &self.ptr
    }
}

unsafe impl IFactory5 for Factory5 {
    unsafe fn raw_f5(&self) -> &IDXGIFactory5 {
        &self.ptr
    }
}

unsafe impl FactoryType for Factory5 {}
