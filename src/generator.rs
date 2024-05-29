use crate::{
    settings::{Settings, SettingsDto, SettingsError},
    utils::set_panic_hook,
};
use chinese_format::{Chinese, ChineseFormat};
use chinese_rand::{ChineseFormatGenerator, FastRandGenerator};
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

type GeneratorFunction = dyn Fn() -> Chinese;

#[wasm_bindgen]
pub struct LogogramGenerator {
    generator_functions: Vec<Box<GeneratorFunction>>,
}

#[wasm_bindgen]
impl LogogramGenerator {
    pub fn try_new(settings_dto: SettingsDto) -> Result<LogogramGenerator, JsValue> {
        set_panic_hook();

        let settings: Settings =
            settings_dto
                .try_into()
                .map_err(|settings_error: SettingsError| {
                    let js_value: JsValue = settings_error.into();
                    js_value
                })?;

        fastrand::seed(settings.seed);

        let chinese_format_generator = {
            fastrand::seed(settings.seed);

            let raw_generator = FastRandGenerator::new();

            Rc::new(ChineseFormatGenerator::new(raw_generator))
        };

        let mut generator_functions: Vec<Box<GeneratorFunction>> = vec![];

        if let Some(integer_range) = settings.integer_range {
            let instance = chinese_format_generator.clone();
            generator_functions.push(Box::new(move || {
                instance
                    .integer(integer_range.clone())
                    .to_chinese(settings.variant.into())
            }))
        }

        if let Some(fraction_settings) = settings.fraction_settings {
            let instance = chinese_format_generator.clone();
            generator_functions.push(Box::new(move || {
                instance
                    .fraction(
                        fraction_settings.denominator_range.clone(),
                        fraction_settings.numerator_range.clone(),
                    )
                    .expect("Denominator is zero by construction")
                    .to_chinese(settings.variant.into())
            }))
        }

        if let Some(count_range) = settings.count_range {
            let instance = chinese_format_generator.clone();
            generator_functions.push(Box::new(move || {
                instance
                    .count(count_range.clone())
                    .to_chinese(settings.variant.into())
            }))
        }

        Ok(Self {
            generator_functions,
        })
    }

    pub fn logograms(&self) -> String {
        //TODO: create a struct for the generator functions
        let generator_function = fastrand::choice(self.generator_functions.iter())
            .expect("There is always at least a function");

        let chinese = generator_function();

        chinese.logograms
    }
}
