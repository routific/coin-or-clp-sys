extern crate coin_or_clp_sys as clp;

use std::ffi::CString;

#[test]
fn read_run_simplex() {
    let model: *mut clp::Clp_Simplex = unsafe { clp::Clp_newModel() };

    let filename = CString::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/p0033.mps"))
        .expect("Failed to build CString");
    let nb_errors = unsafe { clp::Clp_readMps(model, filename.as_ptr(), 0, 0) };
    assert_eq!(nb_errors, 0);

    unsafe {
        clp::Clp_primal(model, 0);
        assert_eq!(clp::Clp_isProvenOptimal(model), 1);
    }
}
