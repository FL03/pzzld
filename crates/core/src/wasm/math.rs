/*
    Appellation: math <module>
    Created At: 2026.01.21:11:20:24
    Contrib: @FL03
*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// increment the given integer by one
pub fn inc(rhs: i32) -> i32 {
    rhs + 1
}

#[wasm_bindgen]
/// decrement the given integer by one
pub fn dec(rhs: i32) -> i32 {
    rhs - 1
}
