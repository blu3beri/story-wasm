#![allow(non_upper_case_globals)]

extern crate wasm_bindgen;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod wasm;

#[macro_use]
mod macros;

pub mod backend;
mod error;
pub mod store;
