//! Rust bindings to the SaxonC library
//!

use std::ffi::{c_char, CStr};

use autocxx::prelude::*;
autocxx::include_cpp! {
    #include "SaxonProcessor.h"
    generate!("SaxonProcessor")
    safety!(unsafe)
    
}





#[cfg(test)]
mod tests {
    
    use std::ffi::{c_char, CStr};

    use autocxx::prelude::*;
    use crate::ffi;

    /// Convierte `*const c_char` a `&'static str`
    fn c_char_to_str(c_str: *const c_char) -> &'static str {
        if c_str.is_null() {
            return "";
        }
        unsafe {
            CStr::from_ptr(c_str)
                .to_str()
                .unwrap_or("")
        }
    }
    
    #[test]
    fn test_version() {
        let mut saxon = ffi::SaxonProcessor::new().within_unique_ptr();
        let version = saxon.pin_mut().version();
        let str_version = c_char_to_str(version);

        assert_eq!("SaxonC-HE 12.5 from Saxonica", str_version);
    }

}
