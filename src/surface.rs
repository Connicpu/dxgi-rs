use error::Error;

use std::mem;
use std::slice;

use winapi::shared::dxgi::{IDXGISurface, DXGI_MAPPED_RECT, DXGI_SURFACE_DESC};
use winapi::shared::dxgiformat::DXGI_FORMAT;
use winapi::shared::minwindef::UINT;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[derive(Clone, PartialEq)]
pub struct Surface {
    ptr: ComPtr<IDXGISurface>,
}

impl Surface {
    #[inline]
    pub fn get_desc(&self) -> SurfaceDesc {
        unsafe {
            let mut desc: SurfaceDesc = mem::uninitialized();
            let hr = self.ptr.GetDesc(&mut desc.desc);
            assert!(SUCCEEDED(hr));
            desc
        }
    }

    #[inline]
    pub unsafe fn map<'a>(
        &'a self,
        read: bool,
        write: bool,
        discard: bool,
    ) -> Result<SurfaceMap<'a>, Error> {
        // TODO: Wait for winapi to be updated with my PR adding these constants
        const DXGI_MAP_READ: UINT = 1;
        const DXGI_MAP_WRITE: UINT = 2;
        const DXGI_MAP_DISCARD: UINT = 4;

        let mut flags = 0;
        if read {
            flags |= DXGI_MAP_READ;
        }
        if write {
            flags |= DXGI_MAP_WRITE;
        }
        if discard {
            flags |= DXGI_MAP_DISCARD;
        }

        let desc = self.get_desc();
        let mut map = mem::uninitialized();
        let hr = self.ptr.Map(&mut map, flags);

        let mut elem_size = 1;
        for i in 2.. {
            if desc.width() * i > map.Pitch as u32 {
                break;
            }
            elem_size = i as usize;
        }

        Error::map_if(hr, || SurfaceMap {
            desc,
            map,
            elem_size,
            surface: &self.ptr,
        })
    }
    
    #[inline]
    pub unsafe fn from_raw(ptr: *mut IDXGISurface) -> Surface {
        Surface {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    #[inline]
    pub unsafe fn get_raw(&self) -> *mut IDXGISurface {
        self.ptr.as_raw()
    }
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

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
    pub fn format(&self) -> DXGI_FORMAT {
        self.desc.Format
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
