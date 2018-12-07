use crate::error::Error;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::Interface;

pub use self::factory::Factory;
pub use self::factory1::Factory1;
pub use self::factory2::Factory2;

pub mod factory;
pub mod factory1;
pub mod factory2;
pub mod factory3;
pub mod factory4;
pub mod factory5;
pub mod factory6;

pub trait FactoryType: ComWrapper + Clone {
    /// Try to cast this factory to a different factory type
    fn try_cast<F: FactoryType>(&self) -> Option<F> {
        unsafe {
            let ptr = self.clone().into_ptr();
            Some(ComWrapper::from_ptr(ptr.cast().ok()?))
        }
    }
}

pub fn create<F: FactoryType>() -> Result<F, Error> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        let hr = load_dxgi_create()(&F::Interface::uuidof(), &mut ptr);
        Error::map_if(hr, || ComWrapper::from_raw(ptr as _))
    }
}

type CreateFn = unsafe extern "system" fn(*const GUID, *mut *mut c_void) -> i32;

#[inline(always)]
#[cfg(not(feature = "support_windows_vista"))]
fn load_dxgi_create() -> CreateFn {
    winapi::shared::dxgi::CreateDXGIFactory1
}

#[cfg(feature = "support_windows_vista")]
fn load_dxgi_create() -> CreateFn {
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::atomic::{AtomicPtr, AtomicUsize};

    use winapi::shared::dxgi::CreateDXGIFactory;
    use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

    const UNINITIALIZED: usize = 0;
    const INIT_IN_PROCESS: usize = 1;
    const INITIALIZED: usize = 2;

    static FN_PTR: AtomicPtr<c_void> = AtomicPtr::new(0 as _);
    static INIT_STATUS: AtomicUsize = AtomicUsize::new(UNINITIALIZED);

    unsafe fn load() -> Option<Create1> {
        // DXGI should already be loaded because of {CreateDXGIFactory}.
        let dxgi = GetModuleHandleA(b"DXGI.DLL\0".as_ptr() as _);
        // But a sanity check just to be safe.
        if dxgi.is_null() {
            return None;
        }

        let func = GetProcAddress(dxgi, b"CreateDXGIFactory1\0".as_ptr() as _);
        convert_fnptr(func as _)
    }

    unsafe fn init() -> Option<Create1> {
        let fnptr = load();
        FN_PTR.store(std::mem::transmute(fnptr), SeqCst);
        INIT_STATUS.store(INITIALIZED, SeqCst);
        fnptr
    }

    unsafe fn wait_init() -> Option<Create1> {
        loop {
            if INIT_STATUS.load(SeqCst) == INITIALIZED {
                return get_fnptr();
            }
        }
    }

    unsafe fn get_fnptr() -> Option<Create1> {
        convert_fnptr(FN_PTR.load(SeqCst))
    }

    unsafe fn convert_fnptr(func: *mut c_void) -> Option<Create1> {
        std::mem::transmute(func)
    }

    unsafe fn try_load() -> Option<CreateFn> {
        unsafe {
            match INIT_STATUS.compare_and_swap(UNINITIALIZED, INIT_IN_PROCESS, SeqCst) {
                UNINITIALIZED => init(),
                INIT_IN_PROCESS => wait_init(),
                INITIALIZED => get_fnptr(),
                _ => unreachable!(),
            }
        }
    }

    try_load().unwrap_or(CreateDXGIFactory)
}
