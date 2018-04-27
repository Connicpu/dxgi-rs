macro_rules! enum_ {
    (
        #[repr($inner:ident)]
        $(#[$attr:meta])*
        pub enum $ety:ident {
            $($(#[$vattr:meta])* $name:ident = $value:expr,)*
        }
    ) => {
        #[repr($inner)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        pub enum $ety {
            $($(#[$vattr])* $name = $value,)*
        }
        impl $ety {
            #[inline]
            pub fn try_from(value: $inner) -> Option<Self> {
                match value {
                    $($value => Some($ety :: $name),)*
                    _ => None,
                }
            }
        }
    }
}
macro_rules! flags {
    (
        #[repr($inner:ident)]
        $(#[$attr:meta])*
        pub enum $flagty:ident {
            $($(#[$vattr:meta])* $name:ident = $value:expr,)*
        }
    ) => {
        #[repr(C)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        $(#[$attr])*
        pub struct $flagty(pub $inner);
        impl $flagty {
            pub const NONE : $flagty = $flagty ( 0 );
            $($(#[$vattr])* pub const $name : $flagty = $flagty ( $value );)*
            #[inline]
            pub fn is_set(self, flag: Self) -> bool {
                self & flag == flag
            }
            #[inline]
            pub fn clear(&mut self, flag: Self) {
                *self &= !flag;
            }
            #[inline]
            pub fn validate(self) -> bool {
                const MASK: $inner = 0 | $($value)|*;
                self.0 & !MASK == 0
            }
        }
        impl $crate::std::ops::Not for $flagty {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                $flagty ( !self.0 )
            }
        }
        impl $crate::std::ops::BitAnd for $flagty {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                $flagty ( self.0 & rhs.0 )
            }
        }
        impl $crate::std::ops::BitAndAssign for $flagty {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
        impl $crate::std::ops::BitOr for $flagty {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                $flagty ( self.0 | rhs.0 )
            }
        }
        impl $crate::std::ops::BitOrAssign for $flagty {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl $crate::std::ops::BitXor for $flagty {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                $flagty ( self.0 ^ rhs.0 )
            }
        }
        impl $crate::std::ops::BitXorAssign for $flagty {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
        impl $crate::std::fmt::Debug for $flagty {
            fn fmt(&self, fmt: &mut $crate::std::fmt::Formatter) -> $crate::std::fmt::Result {
                fmt.write_str(concat!(stringify!($flagty), "("))?;
                let mut first = true;
                $(if self.is_set($flagty :: $name) {
                    if first {
                        first = false;
                    } else {
                        fmt.write_str(" | ")?;
                    }
                    fmt.write_str(stringify!($name))?;
                })*
                if first {
                    fmt.write_str("NONE")?;
                }
                fmt.write_str(")")?;
                Ok(())
            }
        }
    }
}

pub use flags::alpha_mode::AlphaMode;
pub use flags::format::Format;
pub use flags::mode_rotation::ModeRotation;
pub use flags::mode_scaling::ModeScaling;
pub use flags::mode_scanline_order::ModeScanlineOrder;
pub use flags::present::PresentFlags;
pub use flags::scaling::Scaling;
pub use flags::swap_chain::SwapChainFlags;
pub use flags::swap_effect::SwapEffect;
pub use flags::usage::UsageFlags;

pub mod alpha_mode;
pub mod format;
pub mod mode_rotation;
pub mod mode_scaling;
pub mod mode_scanline_order;
pub mod present;
pub mod scaling;
pub mod swap_chain;
pub mod swap_effect;
pub mod usage;
