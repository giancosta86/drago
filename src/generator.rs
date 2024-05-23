use crate::{settings::LogogramGeneratorSettings, utils::set_panic_hook};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
struct LogogramGenerator(LogogramGeneratorSettings);

#[wasm_bindgen]
impl LogogramGenerator {
    #[wasm_bindgen]
    pub fn new(settings: LogogramGeneratorSettings) -> Self {
        set_panic_hook();
        Self(settings)
    }

    #[wasm_bindgen]
    pub fn logograms(&self) -> Result<String, String> {
        Ok("Welcome to the logogram generator! ☺️".to_string())
    }
}
