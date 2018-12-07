#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Luid {
    low_part: u32,
    high_part: i32,
}

impl Luid {
    pub fn as_i64(&self) -> i64 {
        (self.low_part as i64) |
        (self.high_part as i64) << 32
    }
}

impl std::fmt::Debug for Luid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Luid(0x{:016X})", self.as_i64() as u64)
    }
}
