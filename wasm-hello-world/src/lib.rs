extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
//abiltiy to import JS native API ** has to match with actual JS WEB MDN API
extern "C" {
    fn alert(s:&str);
}

#[wasm_bindgen]
//native rust environment
pub fn run_alert(item:&str){
    alert(&format!("this is WASm and {}",item));
}
