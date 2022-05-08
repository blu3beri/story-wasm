#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate wasm_bindgen;

#[macro_use]
mod wasm;

#[macro_use]
mod macros;

pub mod backend;
mod error;
pub mod store;
