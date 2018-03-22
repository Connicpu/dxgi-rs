use error::Error;

use std::mem;
use std::slice;

use winapi::shared::dxgiformat::DXGI_FORMAT;
use winapi::shared::dxgi::{IDXGISurface, DXGI_MAPPED_RECT, DXGI_SURFACE_DESC};
use winapi::shared::minwindef::UINT;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

pub struct Surface {
    ptr: ComPtr<IDXGISurface>,
}

impl Surface {
    pub unsafe fn from_raw(ptr: *mut IDXGISurface) -> Surface {
        Surface {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDXGISurface {
        self.ptr.as_raw()
    }

    pub fn get_desc(&self) -> SurfaceDesc {
        unsafe {
            let mut desc: SurfaceDesc = mem::uninitialized();
            let hr = self.ptr.GetDesc(&mut desc.desc);
            assert!(SUCCEEDED(hr));
            desc
        }
    }

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
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

pub struct SurfaceDesc {
    desc: DXGI_SURFACE_DESC,
}

impl SurfaceDesc {
    pub fn width(&self) -> u32 {
        self.desc.Width
    }

    pub fn height(&self) -> u32 {
        self.desc.Height
    }

    pub fn format(&self) -> DXGI_FORMAT {
        self.desc.Format
    }

    pub fn sample_count(&self) -> u32 {
        self.desc.SampleDesc.Count
    }

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
    pub fn row<T>(&self, row: u32) -> &[T]
    where
        T: Copy,
    {
        unsafe {
            assert!(row < self.desc.height());
            let len = (self.desc.width() as usize * self.elem_size) / mem::size_of::<T>();
            slice::from_raw_parts(self.map.pBits as *mut _, len)
        }
    }

    pub fn row_mut<T>(&mut self, row: u32) -> &mut [T]
    where
        T: Copy,
    {
        unsafe {
            assert!(row < self.desc.height());
            let len = (self.desc.width() as usize * self.elem_size) / mem::size_of::<T>();
            slice::from_raw_parts_mut(self.map.pBits as *mut _, len)
        }
    }
}

impl<'a> Drop for SurfaceMap<'a> {
    fn drop(&mut self) {
        unsafe {
            self.surface.Unmap();
        }
    }
}
