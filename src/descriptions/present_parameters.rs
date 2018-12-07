use math2d::Point2i;
use math2d::Recti;
use winapi::shared::dxgi1_2::DXGI_PRESENT_PARAMETERS;

pub struct PresentParameters<'a> {
    pub dirty_rects: Option<&'a [Recti]>,
    pub scroll_rect: Option<&'a Recti>,
    pub scroll_offset: Option<&'a Point2i>,
}

impl<'a> From<&'a PresentParameters<'a>> for DXGI_PRESENT_PARAMETERS {
    fn from(params: &'a PresentParameters<'a>) -> DXGI_PRESENT_PARAMETERS {
        DXGI_PRESENT_PARAMETERS {
            DirtyRectsCount: params.dirty_rects.map(|r| r.len() as u32).unwrap_or(0),
            pDirtyRects: params
                .dirty_rects
                .map(|r| r.as_ptr() as _)
                .unwrap_or(std::ptr::null_mut()),
            pScrollRect: params
                .scroll_rect
                .map(|r| r as *const _ as _)
                .unwrap_or(std::ptr::null_mut()),
            pScrollOffset: params
                .scroll_offset
                .map(|r| r as *const _ as _)
                .unwrap_or(std::ptr::null_mut()),
        }
    }
}
