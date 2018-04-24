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
            pub fn try_from(value: $inner) -> Option<Self> {
                match value {
                    $($value => Some($ety :: $name),)*
                    _ => None,
                }
            }
        }
    }
}

pub use flags::format::Format;

pub mod format;
