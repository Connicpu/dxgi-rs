pub use self::optional_fn::OptionalFn;

mod optional_fn;

#[cfg(test)]
pub struct StructSizeTracker {
    pub size: usize,
    pub align: usize,
}

#[cfg(test)]
impl StructSizeTracker {
    pub fn new() -> Self {
        StructSizeTracker { size: 0, align: 0 }
    }

    fn realign(&mut self, align: usize) {
        assert_eq!(align.count_ones(), 1);
        if self.size & (align - 1) != 0 {
            let old_size = self.size;
            self.size += align;
            self.size &= !(align - 1);
            println!("realign: {} => {}", old_size, self.size);
        }
    }

    pub fn incr(&mut self, size: usize, align: usize) {
        self.realign(align);
        let old_size = self.size;
        self.size += size;
        self.align = std::cmp::max(self.align, align);
        println!("adding {}, size: {} => {}", size, old_size, self.size);
    }

    pub fn finalize(&mut self) {
        let align = self.align;
        self.realign(align);
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! member_compat_test {
    (
        $testname:ident :
        $t1:ident <=> $t2:ty {
            $( $($m1:ident).+ <=> $m2:ident, )*
        }
    ) => {
        #[test]
        fn $testname() {
            use std::mem::{align_of, size_of};
            assert_eq!(size_of::<$t1>(), size_of::<$t2>(), "types are not the same size");
            assert_eq!(align_of::<$t1>(), align_of::<$t2>(), "types are not the same alignment");
            let mut tracker = crate::helpers::StructSizeTracker::new();
            $(
                member_compat_test!(@member tracker, $t1, $t2, $($m1).+, $m2);
            )*
            tracker.finalize();
            assert_eq!(
                tracker.size,
                size_of::<$t1>(),
                concat!("(tracker.size == size_of::<", stringify!($t1), ">())"),
            );
            assert_eq!(
                tracker.align,
                align_of::<$t1>(),
                concat!("(tracker.align == align_of::<", stringify!($t1), ">())"),
            );
        }
    };
    (@member $tracker:ident, $t1:ty, $t2:ty, $($m1:ident).+, $m2:ident) => {
        unsafe {
            use std::mem::{align_of_val, size_of_val, transmute, zeroed};
            let f1: &$t1 = &zeroed();
            {let f2: &$t2 = transmute(f1);
            assert_eq!(
                (&f1.$($m1).+) as *const _ as *const u8,
                (&f2.$m2) as *const _ as *const u8,
                concat!("(&", stringify!($($m1).+), " == &", stringify!($m2), ")"),
            );
            assert_eq!(
                size_of_val(&f1.$($m1).+),
                size_of_val(&f2.$m2),
                concat!(
                    "(size_of_val(&",
                    stringify!($($m1).+),
                    ") == size_of_val(&",
                    stringify!($m2),
                    "))"
                ),
            );
            assert_eq!(
                align_of_val(&f1.$($m1).+),
                align_of_val(&f2.$m2),
                concat!(
                    "(align_of_val(&",
                    stringify!($($m1).+),
                    ") == align_of_val(&",
                    stringify!($m2),
                    "))"
                ),
            );
            $tracker.incr(size_of_val(&f1.$($m1).+), align_of_val(&f1.$($m1).+));}
            std::mem::forget(f1);
        }
    };
}

pub fn wstrlens(pwstr: &[u16]) -> usize {
    let mut len = 0;
    for &c in pwstr {
        if c == 0 {
            break;
        }
        len += 1;
    }
    len
}

pub struct MemoryDbgHelper(pub u64);

impl std::fmt::Debug for MemoryDbgHelper {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        static LEVELS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];

        let mut amount = self.0 as f64;
        let mut level = 0;
        for _ in 0..LEVELS.len() {
            if amount < 1024.0 {
                break;
            }

            level += 1;
            amount /= 1024.0;
        }

        if level > 0 && amount < 10.0 {
            write!(fmt, "{:.2}{}", amount, LEVELS[level])
        } else if level > 0 && amount < 100.0 {
            write!(fmt, "{:.1}{}", amount, LEVELS[level])
        } else {
            write!(fmt, "{:.0}{}", amount, LEVELS[level])
        }
    }
}

#[test]
fn memory_dbg_helper() {
    assert_eq!(format!("{:?}", MemoryDbgHelper(1024u64.pow(0) * 1)), "1B");
    assert_eq!(format!("{:?}", MemoryDbgHelper(1024u64.pow(0) * 10)), "10B");
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(0) * 100)),
        "100B"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(1) * 1)),
        "1.00KB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(1) * 10)),
        "10.0KB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(1) * 100)),
        "100KB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(2) * 1)),
        "1.00MB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(2) * 10)),
        "10.0MB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(2) * 100)),
        "100MB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(3) * 1)),
        "1.00GB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(3) * 10)),
        "10.0GB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(3) * 100)),
        "100GB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(4) * 1)),
        "1.00TB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(4) * 10)),
        "10.0TB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(4) * 100)),
        "100TB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(5) * 1)),
        "1.00PB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(5) * 10)),
        "10.0PB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(5) * 100)),
        "100PB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(6) * 1)),
        "1.00EB"
    );
    assert_eq!(
        format!("{:?}", MemoryDbgHelper(1024u64.pow(6) * 10)),
        "10.0EB"
    );
    assert_eq!(format!("{:?}", MemoryDbgHelper(std::u64::MAX)), "16.0EB");
}
