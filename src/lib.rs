//! # Coin-OR Clp library bindings
//!
//! This crate provides bindings to the [Coin-OR] LP solver ([CLP]) library. The bindings are
//! generated using [bindgen]. CLP is implemented in C++ but those bindings maps to the provided [C
//! interface] of CLP.
//!
//! Usage of this crate creates runtime dependency on the CLP C++ library so be sure to install
//! that (for example using [coinbrew]).
//!
//! [Coin-OR]: https://www.coin-or.org
//! [CLP]: https://github.com/coin-or/Clp
//! [bindgen]: https://crates.io/crates/bindgen
//! [C interface]: https://github.com/coin-or/Clp/blob/master/Clp/src/Clp_C_Interface.h
//! [coinbrew]: https://github.com/coin-or/coinbrew

#[allow(non_camel_case_types, unused_imports, non_upper_case_globals)]
mod ffi;

pub use crate::ffi::*;
