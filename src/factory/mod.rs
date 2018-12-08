use crate::error::Error;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::TYPE_E_DLLFUNCTIONNOTFOUND;
use winapi::Interface;

pub use self::factory::Factory;
pub use self::factory1::Factory1;
pub use self::factory2::Factory2;
pub use self::factory3::Factory3;
pub use self::factory4::Factory4;
pub use self::factory5::Factory5;
pub use self::factory6::Factory6;

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
        let hr = if let Some(create2) = load_dxgi_create2() {
            create2(0, &F::Interface::uuidof(), &mut ptr)
        } else {
            load_dxgi_create()(&F::Interface::uuidof(), &mut ptr)
        };
        Error::map_if(hr, || ComWrapper::from_raw(ptr as _))
    }
}

pub fn create_debug<F: FactoryType>() -> Result<F, Error> {
    unsafe {
        let mut ptr = std::ptr::null_mut();
        if let Some(create) = load_dxgi_create2() {
            init_dxgidebug();
            let hr = create(1, &F::Interface::uuidof(), &mut ptr);
            Error::map_if(hr, || ComWrapper::from_raw(ptr as _))
        } else {
            Err(Error(TYPE_E_DLLFUNCTIONNOTFOUND))
        }
    }
}

type CreateFn = unsafe extern "system" fn(*const GUID, *mut *mut c_void) -> i32;
type Create2Fn = unsafe extern "system" fn(u32, *const GUID, *mut *mut c_void) -> i32;

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
    const INIT_IN_PROGRESS: usize = 1;
    const INITIALIZED: usize = 2;

    static FN_PTR: AtomicPtr<c_void> = AtomicPtr::new(0 as _);
    static INIT_STATUS: AtomicUsize = AtomicUsize::new(UNINITIALIZED);

    unsafe fn load() -> Option<CreateFn> {
        // DXGI should already be loaded because of {CreateDXGIFactory}.
        let dxgi = GetModuleHandleA(b"DXGI.DLL\0".as_ptr() as _);
        // But a sanity check just to be safe.
        if dxgi.is_null() {
            return None;
        }

        let func = GetProcAddress(dxgi, b"CreateDXGIFactory1\0".as_ptr() as _);
        convert_fnptr(func as _)
    }

    unsafe fn init() -> Option<CreateFn> {
        let fnptr = load();
        FN_PTR.store(std::mem::transmute(fnptr), SeqCst);
        INIT_STATUS.store(INITIALIZED, SeqCst);
        fnptr
    }

    unsafe fn wait_init() -> Option<CreateFn> {
        loop {
            if INIT_STATUS.load(SeqCst) == INITIALIZED {
                return get_fnptr();
            }
        }
    }

    unsafe fn get_fnptr() -> Option<CreateFn> {
        convert_fnptr(FN_PTR.load(SeqCst))
    }

    unsafe fn convert_fnptr(func: *mut c_void) -> Option<CreateFn> {
        std::mem::transmute(func)
    }

    unsafe fn try_load() -> Option<CreateFn> {
        match INIT_STATUS.compare_and_swap(UNINITIALIZED, INIT_IN_PROGRESS, SeqCst) {
            UNINITIALIZED => init(),
            INIT_IN_PROGRESS => wait_init(),
            INITIALIZED => get_fnptr(),
            _ => unreachable!(),
        }
    }

    unsafe { try_load() }.unwrap_or(CreateDXGIFactory)
}

fn load_dxgi_create2() -> Option<Create2Fn> {
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::atomic::{AtomicPtr, AtomicUsize};

    use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

    const UNINITIALIZED: usize = 0;
    const INIT_IN_PROGRESS: usize = 1;
    const INITIALIZED: usize = 2;

    static FN_PTR: AtomicPtr<c_void> = AtomicPtr::new(0 as _);
    static INIT_STATUS: AtomicUsize = AtomicUsize::new(UNINITIALIZED);

    unsafe fn load() -> Option<Create2Fn> {
        // DXGI should already be loaded because of {CreateDXGIFactory}.
        let dxgi = GetModuleHandleA(b"DXGI.DLL\0".as_ptr() as _);
        // But a sanity check just to be safe.
        if dxgi.is_null() {
            return None;
        }

        let func = GetProcAddress(dxgi, b"CreateDXGIFactory2\0".as_ptr() as _);
        convert_fnptr(func as _)
    }

    unsafe fn init() -> Option<Create2Fn> {
        let fnptr = load();
        FN_PTR.store(std::mem::transmute(fnptr), SeqCst);
        INIT_STATUS.store(INITIALIZED, SeqCst);
        fnptr
    }

    unsafe fn wait_init() -> Option<Create2Fn> {
        loop {
            if INIT_STATUS.load(SeqCst) == INITIALIZED {
                return get_fnptr();
            }
        }
    }

    unsafe fn get_fnptr() -> Option<Create2Fn> {
        convert_fnptr(FN_PTR.load(SeqCst))
    }

    unsafe fn convert_fnptr(func: *mut c_void) -> Option<Create2Fn> {
        std::mem::transmute(func)
    }

    unsafe fn try_load() -> Option<Create2Fn> {
        match INIT_STATUS.compare_and_swap(UNINITIALIZED, INIT_IN_PROGRESS, SeqCst) {
            UNINITIALIZED => init(),
            INIT_IN_PROGRESS => wait_init(),
            INITIALIZED => get_fnptr(),
            _ => unreachable!(),
        }
    }

    unsafe { try_load() }
}

fn init_dxgidebug() {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::SeqCst;

    use winapi::um::libloaderapi::LoadLibraryA;

    const UNINITIALIZED: usize = 0;
    const INIT_IN_PROGRESS: usize = 1;
    const INITIALIZED: usize = 2;

    static INIT_STATUS: AtomicUsize = AtomicUsize::new(0);

    unsafe fn init() {
        LoadLibraryA(b"DXGIDEBUG.DLL\0".as_ptr() as _);
        INIT_STATUS.store(INITIALIZED, SeqCst);
    }

    unsafe fn wait_init() {
        loop {
            if INIT_STATUS.load(SeqCst) == INITIALIZED {
                return;
            }
        }
    }

    unsafe fn try_init() {
        match INIT_STATUS.compare_and_swap(UNINITIALIZED, INIT_IN_PROGRESS, SeqCst) {
            UNINITIALIZED => init(),
            INIT_IN_PROGRESS => wait_init(),
            INITIALIZED => return,
            _ => unreachable!(),
        }
    }

    unsafe { try_init() }
}
