fn main() {
    {
        use std::os::raw::c_char;
        use std::ffi::CString;

        extern {
            fn strlen(s: *const c_char) -> usize;
        }

        let rust_str = "I'll be back";
        let null_terminated = CString::new(rust_str).unwrap();
        unsafe {
            assert_eq!(strlen(null_terminated.as_ptr()), 12);
        }
    }
}
