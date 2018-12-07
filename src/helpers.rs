use std::ops::Deref;

use com_wrapper::ComWrapper;

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
            let mut tracker = ::helpers::StructSizeTracker::new();
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

pub unsafe fn deref_com_wrapper<T, U>(wrapper: &T) -> &U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>(),
    );
    assert_eq!(
        std::mem::size_of::<U>(),
        std::mem::size_of::<*mut U::Interface>(),
    );

    std::mem::transmute::<&T, &U>(wrapper)
}

pub unsafe fn deref_com_wrapper_mut<T, U>(wrapper: &mut T) -> &mut U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(std::mem::size_of::<U>(), std::mem::size_of::<T>());
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>(),
    );
    assert_eq!(
        std::mem::size_of::<U>(),
        std::mem::size_of::<*mut U::Interface>(),
    );

    std::mem::transmute::<&mut T, &mut U>(wrapper)
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
