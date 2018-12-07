use crate::adapter::Adapter;
use crate::error::Error;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIAdapter1, IDXGIDevice};
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The DXGI Device interface is designed for use by DXGI objects that need
/// access to other DXGI objects. This interface is useful to applications
/// that do not use Direct3D to communicate with DXGI.
///
/// This type does not provide any behavior on its own, but rather is used more
/// like a handle that is required to be passed when creating types for other
/// objects in the DirectX family of APIs, such as [`direct2d::Device`][1]. The
/// simplest route for obtaining a `Device` is to create a
/// [`direct3d11::Device`][2] and use the [`as_dxgi`][3] method to cast it to
/// this type.
///
/// **Windows Phone 8:** This API is supported.
///
/// [1]: https://docs.rs/direct2d/*/direct2d/struct.Device.html
/// [2]: https://docs.rs/direct3d11/*/direct3d11/device/struct.Device.html
/// [3]: https://docs.rs/direct3d11/*/direct3d11/device/struct.Device.html#method.as_dxgi
pub struct Device {
    ptr: ComPtr<IDXGIDevice>,
}

impl Device {
    /// Returns the adapter associated with this device.
    #[inline]
    pub fn adapter(&self) -> Result<Adapter, Error> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetAdapter(&mut ptr);
            Error::map(hr, ())?;
            let mut ptr1 = ptr::null_mut();
            let hr = (*ptr).QueryInterface(&IDXGIAdapter1::uuidof(), &mut ptr1);
            Error::map_if(hr, || Adapter::from_raw(ptr1 as *mut _))
        }
    }
}
