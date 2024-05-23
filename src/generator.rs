use crate::{settings::LogogramGeneratorSettings, utils::set_panic_hook};
use chinese_format::{
    gregorian::{DatePattern, WeekFormat},
    ChineseFormat,
};
use chinese_rand::{gregorian::DateParams, ChineseFormatGenerator, FastRandGenerator};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct LogogramGenerator {
    settings: LogogramGeneratorSettings,
}

#[wasm_bindgen]
impl LogogramGenerator {
    #[wasm_bindgen]
    pub fn create(settings: LogogramGeneratorSettings) -> Self {
        set_panic_hook();
        Self { settings }
    }

    #[wasm_bindgen]
    pub fn logograms(&self) -> Result<String, String> {
        let chinese_format_generator = ChineseFormatGenerator::new(FastRandGenerator::new());

        Ok(format!(
            "Greetings! {}",
            chinese_format_generator
                .gregorian()
                .date(DateParams {
                    year_range: None,
                    formal: true,
                    pattern: DatePattern::YearMonthDayWeekDay,
                    week_format: Some(WeekFormat::Zhou)
                })
                .to_chinese(self.settings.variant.into())
        ))
    }
}
