#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Luid {
    low_part: u32,
    high_part: i32,
}

impl Luid {
    pub fn as_i64(&self) -> i64 {
        (self.low_part as i64) | (self.high_part as i64) << 32
    }
}

impl std::fmt::Debug for Luid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Luid(0x{:016X})", self.as_i64() as u64)
    }
}

impl From<i64> for Luid {
    fn from(i: i64) -> Luid {
        Luid {
            low_part: i as u32,
            high_part: (i >> 32) as i32,
        }
    }
}

impl From<Luid> for i64 {
    fn from(l: Luid) -> i64 {
        l.as_i64()
    }
}

impl From<winapi::shared::ntdef::LUID> for Luid {
    fn from(l: winapi::shared::ntdef::LUID) -> Luid {
        Luid {
            high_part: l.HighPart,
            low_part: l.LowPart,
        }
    }
}

impl From<Luid> for winapi::shared::ntdef::LUID {
    fn from(l: Luid) -> Self {
        Self {
            HighPart: l.high_part,
            LowPart: l.low_part,
        }
    }
}
