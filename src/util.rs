use std::ffi::CString;

//macro_rules! c_str {
//  ($s:expr) => (
//    concat!($s, "\0") as *const str as *const [i8] as *const i8
//  );
//}

pub fn c_str(s: &str) -> CString {
    CString::new(s).unwrap()
}
