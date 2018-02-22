#![macro_use]

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

//pub fn c_str(s: &str) -> CString {
//    CString::new(s).unwrap()
//}
