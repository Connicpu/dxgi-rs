use crate::descriptions::SurfaceDesc;
use crate::enums::MapFlags;
use dcommon::error::Error;

use std::mem::MaybeUninit;

use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGISurface, DXGI_MAPPED_RECT};
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Surface {
    ptr: ComPtr<IDXGISurface>,
}

pub unsafe trait ISurface {
    /// Get a descriptor of this surface.
    fn desc(&self) -> SurfaceDesc {
        unsafe {
            let mut desc = MaybeUninit::uninit();
            let hr = self.raw_surface().GetDesc(desc.as_mut_ptr());
            assert!(SUCCEEDED(hr));
            desc.assume_init().into()
        }
    }

    /// Map a surface so that its memory may be read from or written to. This
    /// can easily be an unsafe operation, so this method and all of the
    /// methods on SurfaceMap are marked unsafe. Be aware that Unmap is called
    /// in SurfaceMap's `Drop` implementation. It is up to the caller to ensure
    /// that this surface is not concurrently mapped twice.
    unsafe fn map<'a>(&'a self, flags: MapFlags) -> Result<SurfaceMap<'a>, Error> {
        let desc = self.desc();
        let mut map = MaybeUninit::uninit();
        let hr = self.raw_surface().Map(map.as_mut_ptr(), flags.0);
        if hr < 0 {
            return Err(Error::from(hr));
        }

        let map = map.assume_init();
        let elem_size = map.Pitch as usize / desc.width as usize;

        Ok(SurfaceMap {
            desc,
            map,
            elem_size,
            surface: &self.raw_surface(),
        })
    }

    unsafe fn raw_surface(&self) -> &IDXGISurface;
}

unsafe impl ISurface for Surface {
    unsafe fn raw_surface(&self) -> &IDXGISurface {
        &self.ptr
    }
}

pub struct SurfaceMap<'a> {
    pub desc: SurfaceDesc,
    map: DXGI_MAPPED_RECT,
    elem_size: usize,
    surface: &'a IDXGISurface,
}

impl<'a> SurfaceMap<'a> {
    /// NOTE: Unsafe because we can't verify T is compatible
    /// with the underlying data.
    ///
    /// Panics if `row` is not less than `desc.height`
    pub unsafe fn row<T>(&self, row: u32) -> &[T]
    where
        T: Copy,
    {
        assert!(row < self.desc.height);
        let len = (self.desc.width as usize * self.elem_size) / std::mem::size_of::<T>();
        let ptr = (self.map.pBits as *mut u8).offset(self.map.Pitch as isize * row as isize);
        std::slice::from_raw_parts(ptr as *mut _, len)
    }

    /// NOTE: Unsafe because we can't verify T is compatible
    /// with the underlying data.
    ///
    /// Panics if `row` is not less than `desc.height`
    pub unsafe fn row_mut<T>(&mut self, row: u32) -> &mut [T]
    where
        T: Copy,
    {
        assert!(row < self.desc.height);
        let len = (self.desc.width as usize * self.elem_size) / std::mem::size_of::<T>();
        let ptr = (self.map.pBits as *mut u8).offset(self.map.Pitch as isize * row as isize);
        std::slice::from_raw_parts_mut(ptr as *mut _, len)
    }
}

impl<'a> Drop for SurfaceMap<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.surface.Unmap();
        }
    }
}
