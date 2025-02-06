// wasm_interface/src/lib.rs
use wasm_bindgen::prelude::*;
use ex::yeah;


#[wasm_bindgen]
pub fn export_to_js() -> String {
    yeah()
}