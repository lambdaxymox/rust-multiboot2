//!
//! Utility functions.
//!
use core::str;


/// The function `validate_c_string` checks that the `string` termintates with
/// a null character. This is essential when casting multiboot strings since 
/// they were created outside our operating environment.
pub fn validate_c_string(string: &str) -> bool {
    if !string.is_empty() {
        return string.bytes().last() == Some(0x00);
    }

    true
}