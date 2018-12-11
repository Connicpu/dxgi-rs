use std::marker::PhantomData;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use winapi::ctypes::c_void;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};

const UNINITIALIZED: usize = 0;
const INIT_IN_PROGRESS: usize = 1;
const INITIALIZED: usize = 2;

pub struct OptionalFn<'a, F> {
    status: AtomicUsize,
    fn_ptr: AtomicPtr<c_void>,
    fn_lib: &'a str,
    fn_name: &'a str,
    _marker: PhantomData<F>,
}

impl<'a, F> OptionalFn<'a, F> {
    pub const fn new(fn_lib: &'a str, fn_name: &'a str) -> Self {
        OptionalFn {
            status: AtomicUsize::new(UNINITIALIZED),
            fn_ptr: AtomicPtr::new(0 as _),
            fn_lib: fn_lib,
            fn_name: fn_name,
            _marker: PhantomData,
        }
    }

    pub unsafe fn get(&self) -> Option<F> {
        self.try_load()
    }

    fn make_cstr(s: &str) -> Vec<u8> {
        s.as_bytes().iter().cloned().chain(Some(0)).collect()
    }

    unsafe fn load(&self) -> *mut c_void {
        let lib_str = Self::make_cstr(self.fn_lib);
        let dxgi = LoadLibraryA(lib_str.as_ptr() as _);
        if dxgi.is_null() {
            return std::ptr::null_mut();
        }

        let name_str = Self::make_cstr(self.fn_name);
        let func = GetProcAddress(dxgi, name_str.as_ptr() as _);
        func as _
    }

    unsafe fn init(&self) -> Option<F> {
        let fnptr = self.load();
        self.fn_ptr.store(fnptr, SeqCst);
        self.status.store(INITIALIZED, SeqCst);
        self.convert_fnptr(fnptr)
    }

    unsafe fn wait_init(&self) -> Option<F> {
        loop {
            if self.status.load(SeqCst) == INITIALIZED {
                return self.get_fnptr();
            }
        }
    }

    unsafe fn get_fnptr(&self) -> Option<F> {
        self.convert_fnptr(self.fn_ptr.load(SeqCst))
    }

    unsafe fn convert_fnptr(&self, func: *mut c_void) -> Option<F> {
        assert_eq!(
            std::mem::size_of::<*mut c_void>(),
            std::mem::size_of::<Option<F>>()
        );
        std::mem::transmute_copy::<*mut c_void, Option<F>>(&func)
    }

    unsafe fn try_load(&self) -> Option<F> {
        match self
            .status
            .compare_and_swap(UNINITIALIZED, INIT_IN_PROGRESS, SeqCst)
        {
            UNINITIALIZED => self.init(),
            INIT_IN_PROGRESS => self.wait_init(),
            INITIALIZED => self.get_fnptr(),
            _ => unreachable!(),
        }
    }
}
