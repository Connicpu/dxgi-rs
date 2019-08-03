#[doc(inline)]
pub use self::output::{IOutput, Output};

mod output;

#[cfg(test)]
mod compile_test {
    #![allow(dead_code)]
    use super::*;

    fn dyn_output(o: &Output) -> &dyn IOutput {
        o
    }
}
