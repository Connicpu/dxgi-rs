use std::ffi::OsString;
use std::fmt;
use std::ptr;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{LANG_NEUTRAL, LPWSTR, MAKELANGID, SUBLANG_NEUTRAL};
use winapi::shared::winerror::{HRESULT_FROM_WIN32, HRESULT, SUCCEEDED};
use winapi::um::winbase::{FormatMessageW, LocalFree, FORMAT_MESSAGE_ALLOCATE_BUFFER,
                          FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS};
use wio::wide::FromWide;

pub struct Error(pub HRESULT);

impl Error {
    pub fn map<T>(hr: HRESULT, success_value: T) -> Result<T, Error> {
        if SUCCEEDED(hr) {
            Ok(success_value)
        } else {
            Err(Error(hr))
        }
    }

    pub fn map_if<F, T>(hr: HRESULT, if_success: F) -> Result<T, Error> where F: FnOnce() -> T {
        if SUCCEEDED(hr) {
            Ok(if_success())
        } else {
            Err(Error(hr))
        }
    }

    pub fn from_win32(err: DWORD) -> Error {
        Error(HRESULT_FROM_WIN32(err))
    }

    pub fn get_message(&self) -> String {
        format_err(self.0)
    }
}

impl From<HRESULT> for Error {
    fn from(hr: HRESULT) -> Error {
        Error(hr)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Error")
            .field(&self.0)
            .field(&self.get_message())
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.get_message())
    }
}

fn format_err(hr: HRESULT) -> String {
    unsafe {
        let flags = FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_IGNORE_INSERTS;

        let mut msg: LPWSTR = ptr::null_mut();
        let len = FormatMessageW(
            flags,
            ptr::null_mut(),
            hr as u32,
            MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL) as u32,
            (&mut msg) as *mut _ as *mut _,
            0,
            ptr::null_mut(),
        );

        if len == 0 {
            return format!("Unknown Error 0x{:x}", hr);
        }

        let os = OsString::from_wide_ptr(msg, len as usize);
        LocalFree(msg as *mut _);

        os.to_string_lossy().into_owned()
    }
}
