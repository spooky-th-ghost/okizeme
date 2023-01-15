#[macro_export]
macro_rules! exports {
    ($($t:tt),+) => {
        $(pub mod $t {
            pub use okizeme_$t::*;
        })*
    }
}
