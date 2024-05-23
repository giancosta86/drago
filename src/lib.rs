mod dtos;
mod utils;

use crate::dtos::GeneratorSettings;
use chinese_format::ChineseFormat;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_number(settings: GeneratorSettings) -> Result<String, String> {
    set_panic_hook();

    /*
    match result {
        Ok(number_string) => Ok(number_string),
        Err(message) => Err(message.to_string()),
    }*/

    Ok(format!(
        "Hello, from my WebAssembly! At last! {} ☺️",
        90.to_chinese(chinese_format::Variant::Simplified)
    ))
}
