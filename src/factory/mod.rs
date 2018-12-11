use crate::error::Error;
use crate::helpers::OptionalFn;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::dxgi::CreateDXGIFactory;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::SUCCEEDED;
use winapi::shared::winerror::TYPE_E_DLLFUNCTIONNOTFOUND;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::Interface;

#[doc(inline)]
pub use self::factory::{AdapterIter, Factory};
#[doc(inline)]
pub use self::factory1::{AdapterIter1, Factory1};
#[doc(inline)]
pub use self::factory2::{Factory2, RegisterStatusToken, StatusEventCookie, StatusEventReceiver};
#[doc(inline)]
pub use self::factory3::Factory3;
#[doc(inline)]
pub use self::factory4::Factory4;
#[doc(inline)]
pub use self::factory5::Factory5;
#[doc(inline)]
pub use self::factory6::{AdapterIterByPreference, Factory6};

mod factory;
mod factory1;
mod factory2;
mod factory3;
mod factory4;
mod factory5;
mod factory6;

pub trait FactoryType: ComWrapper + Clone {
    /// Try to cast this factory to a different factory type
    fn try_cast<F: FactoryType>(&self) -> Option<F> {
        unsafe {
            let ptr = self.clone().into_ptr();
            Some(ComWrapper::from_ptr(ptr.cast().ok()?))
        }
    }
}

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
