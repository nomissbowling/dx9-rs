//! cppbridge bindings
//!
//! to control publication included bridge_bindings.rs

// #![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// #![allow(missing_unsafe_on_extern)] // needless on bindgen 0.72.1
#![allow(unsafe_op_in_unsafe_fn)]
include!(concat!("../../../include/", "bridge_bindings.rs"));
