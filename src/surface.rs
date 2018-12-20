use crate::enums::{Format, MapFlags};
use dcommon::error::Error;

use std::mem;
use std::slice;

use checked_enum::UncheckedEnum;
use winapi::shared::dxgi::{IDXGISurface, DXGI_MAPPED_RECT, DXGI_SURFACE_DESC};
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Surface {
    ptr: ComPtr<IDXGISurface>,
}

impl Surface {
    #[inline]
    /// Get a descriptor of this surface.
    pub fn desc(&self) -> SurfaceDesc {
        unsafe {
            let mut desc: SurfaceDesc = mem::uninitialized();
            let hr = self.ptr.GetDesc(&mut desc.desc);
            assert!(SUCCEEDED(hr));
            desc
        }
    }

    #[inline]
    /// Map a surface so that its memory may be read from or written to. This
    /// can easily be an unsafe operation, so this method and all of the
    /// methods on SurfaceMap are marked unsafe. Be aware that Unmap is called
    /// in SurfaceMap's `Drop` implementation. It is up to the caller to ensure
    /// that this surface is not concurrently mapped twice.
    pub unsafe fn map<'a>(&'a self, flags: MapFlags) -> Result<SurfaceMap<'a>, Error> {
        let desc = self.desc();
        let mut map = mem::uninitialized();
        let hr = self.ptr.Map(&mut map, flags.0);

        let mut elem_size = 1;
        for i in 2.. {
            if desc.width() * i > map.Pitch as u32 {
                break;
            }
            elem_size = i as usize;
        }

        Error::map_if(hr, move || SurfaceMap {
            desc,
            map,
            elem_size,
            surface: &self.ptr,
        })
    }
}

#[derive(Copy, Clone)]
pub struct SurfaceDesc {
    desc: DXGI_SURFACE_DESC,
}

impl SurfaceDesc {
    #[inline]
    pub fn width(&self) -> u32 {
        self.desc.Width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.desc.Height
    }

    #[inline]
    pub fn format(&self) -> UncheckedEnum<Format> {
        self.desc.Format.into()
    }

    #[inline]
    pub fn sample_count(&self) -> u32 {
        self.desc.SampleDesc.Count
    }

    #[inline]
    pub fn sample_quality(&self) -> u32 {
        self.desc.SampleDesc.Quality
    }
}

pub struct SurfaceMap<'a> {
    pub desc: SurfaceDesc,
    map: DXGI_MAPPED_RECT,
    elem_size: usize,
    surface: &'a IDXGISurface,
}

impl<'a> SurfaceMap<'a> {
    #[inline]
    pub unsafe fn row<T>(&self, row: u32) -> &[T]
    where
        T: Copy,
    {
        assert!(row < self.desc.height());
        let len = (self.desc.width() as usize * self.elem_size) / mem::size_of::<T>();
        let ptr = (self.map.pBits as *mut u8).offset(self.map.Pitch as isize * row as isize);
        slice::from_raw_parts(ptr as *mut _, len)
    }

    #[inline]
    pub unsafe fn row_mut<T>(&mut self, row: u32) -> &mut [T]
    where
        T: Copy,
    {
        assert!(row < self.desc.height());
        let len = (self.desc.width() as usize * self.elem_size) / mem::size_of::<T>();
        let ptr = (self.map.pBits as *mut u8).offset(self.map.Pitch as isize * row as isize);
        slice::from_raw_parts_mut(ptr as *mut _, len)
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
