use crate::helpers::OptionalFn;
use dcommon::error::Error;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::dxgi::CreateDXGIFactory;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::SUCCEEDED;
use winapi::shared::winerror::TYPE_E_DLLFUNCTIONNOTFOUND;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::Interface;

#[doc(inline)]
pub use self::factory::{AdapterIter, Factory, IFactory};
#[doc(inline)]
pub use self::factory1::{AdapterIter1, Factory1, IFactory1};
#[doc(inline)]
pub use self::factory2::{
    Factory2, IFactory2, RegisterStatusToken, StatusEventCookie, StatusEventReceiver,
};
#[doc(inline)]
pub use self::factory3::{Factory3, IFactory3};
#[doc(inline)]
pub use self::factory4::{Factory4, IFactory4};
#[doc(inline)]
pub use self::factory5::{Factory5, IFactory5};
#[doc(inline)]
pub use self::factory6::{AdapterIterByPreference, Factory6, IFactory6};

mod factory;
mod factory1;
mod factory2;
mod factory3;
mod factory4;
mod factory5;
mod factory6;

pub mod traits {
    pub use super::{IFactory, IFactory1, IFactory2, IFactory3, IFactory4, IFactory5, IFactory6};
}

pub unsafe trait FactoryType: ComWrapper + Clone {}

static CREATE_1: OptionalFn<CreateFn> = OptionalFn::new("DXGI.DLL", "CreateDXGIFactory1");
static CREATE_2: OptionalFn<Create2Fn> = OptionalFn::new("DXGI.DLL", "CreateDXGIFactory2");

pub fn create<F: FactoryType>() -> Result<F, Error> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        let hr = if let Some(create1) = CREATE_1.get() {
            create1(&F::Interface::uuidof(), &mut ptr)
        } else {
            CreateDXGIFactory(&F::Interface::uuidof(), &mut ptr)
        };
        Error::map_if(hr, || ComWrapper::from_raw(ptr as _))
    }
}

pub fn create_debug<F: FactoryType>() -> Result<F, Error> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        if let Some(create) = CREATE_2.get() {
            init_dxgidebug();
            let hr = create(1, &F::Interface::uuidof(), &mut ptr);
            Error::map_if(hr, || ComWrapper::from_raw(ptr as _))
        } else {
            Err(Error(TYPE_E_DLLFUNCTIONNOTFOUND))
        }
    }
}

static DECLARE_ADAPTER_REMOVAL_SUPPORT: OptionalFn<DeclArSupFn> =
    OptionalFn::new("DXGI.DLL", "DXGIDeclareAdapterRemovalSupport");

pub fn declare_adapter_removal_support() -> Option<bool> {
    unsafe {
        DECLARE_ADAPTER_REMOVAL_SUPPORT
            .get()
            .map(|dars| SUCCEEDED(dars()))
    }
}

type CreateFn = unsafe extern "system" fn(*const GUID, *mut *mut c_void) -> i32;
type Create2Fn = unsafe extern "system" fn(u32, *const GUID, *mut *mut c_void) -> i32;
type DeclArSupFn = unsafe extern "system" fn() -> i32;

fn init_dxgidebug() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| unsafe {
        LoadLibraryA(b"DXGIDEBUG.DLL\0".as_ptr() as _);
    });
}

#[cfg(test)]
mod compile_test {
    #![allow(dead_code)]
    use super::{traits::*, Factory6};

    fn dyn_factory(f: &Factory6) -> &dyn IFactory {
        f
    }
    fn dyn_factory1(f: &Factory6) -> &dyn IFactory1 {
        f
    }
    fn dyn_factory2(f: &Factory6) -> &dyn IFactory2 {
        f
    }
    fn dyn_factory3(f: &Factory6) -> &dyn IFactory3 {
        f
    }
    fn dyn_factory4(f: &Factory6) -> &dyn IFactory4 {
        f
    }
    fn dyn_factory5(f: &Factory6) -> &dyn IFactory5 {
        f
    }
    fn dyn_factory6(f: &Factory6) -> &dyn IFactory6 {
        f
    }
}
