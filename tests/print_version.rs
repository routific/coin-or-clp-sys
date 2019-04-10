extern crate coin_or_clp_sys;

use std::ffi::CStr;

#[test]
fn print_version() {
    let version = unsafe { CStr::from_ptr(coin_or_clp_sys::Clp_Version()).to_string_lossy() };
    println!("version: {}", version);
}
