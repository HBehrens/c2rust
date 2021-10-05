//! extern_crate_memoffset

extern crate libc;

use variable_offsetof::{rust_get_offset, rust_get_offset2, rust_get_offset_nested, size_t};

#[link(name = "test")]
extern "C" {
    #[no_mangle]
    fn get_offset(_: size_t) -> size_t;
    #[no_mangle]
    fn get_offset2(_: size_t) -> size_t;
    #[no_mangle]
    fn get_offset_nested(_: size_t) -> size_t;
}

pub fn test_get_offset() {
    for idx in 0..3 {
        {
            let rust_ret = unsafe { rust_get_offset(idx) };
            let c_ret = unsafe { get_offset(idx) };

            assert_eq!(rust_ret, c_ret);
        }

        {
            let rust_ret = unsafe { rust_get_offset2(idx) };
            let c_ret = unsafe { get_offset2(idx) };

            assert_eq!(rust_ret, c_ret);
        }

        {
            let rust_ret = unsafe { rust_get_offset_nested(idx) };
            let c_ret = unsafe { get_offset_nested(idx) };

            assert_eq!(rust_ret, c_ret);
        }
    }
}
