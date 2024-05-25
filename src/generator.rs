use crate::{settings::LogogramGeneratorSettings, utils::set_panic_hook};
use chinese_format::{
    gregorian::{DatePattern, WeekFormat},
    Chinese, ChineseFormat,
};
use chinese_rand::{gregorian::DateParams, ChineseFormatGenerator, FastRandGenerator};
use wasm_bindgen::prelude::wasm_bindgen;

type GeneratorFunction = fn() -> Chinese;

#[wasm_bindgen]
pub struct LogogramGenerator {
    generator_closures: Vec<GeneratorFunction>,
}

#[wasm_bindgen]
impl LogogramGenerator {
    #[wasm_bindgen]
    pub fn try_new(settings: LogogramGeneratorSettings) -> Result<LogogramGenerator, String> {
        set_panic_hook();

        Ok(Self {
            generator_closures: vec![],
        })
    }

    #[wasm_bindgen]
    pub fn logograms(&self) -> String {
        let chinese_format_generator = ChineseFormatGenerator::new(FastRandGenerator::new());

        format!("Greetings! {}", "☺️")
    }
}
