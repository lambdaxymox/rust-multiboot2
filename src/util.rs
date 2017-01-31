//!
//! Utility functions.
//!
use core::str;


/// The function `validate_c_string` checks that the `string` termintates with
/// a null character, and  that null characters do not appear elsewhere in the string. 
/// This is essential when casting multiboot strings since they were created 
/// outside our operating environment.
pub fn validate_cstring(string: &str) -> bool {
    let mut bytes_rev = string.bytes().rev();

    // The string does not terminate with a null character.
    if bytes_rev.next() != Some(0x00) {
        return false;
    }

    for ch in bytes_rev {
        if ch == 0x00 {
            return false;
        }
    }

    true
}