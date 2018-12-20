pub use self::allow_tearing::AllowTearing;

mod allow_tearing;

pub unsafe trait Feature {
    const FLAG: u32;

    type Structure: Sized;
    type Result;

    fn get_result(hr: i32, structure: &Self::Structure) -> Self::Result;
}
