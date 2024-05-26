use crate::{error::ErrorCode, settings::LogogramGeneratorSettings, utils::set_panic_hook};
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
    #[wasm_bindgen]
    pub fn try_new(settings: LogogramGeneratorSettings) -> Result<LogogramGenerator, JsValue> {
        set_panic_hook();

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
                    .integer((&integer_range).into())
                    .to_chinese(settings.variant.into())
            }))
        }

        if let Some(fraction_settings) = settings.fraction_settings {
            if fraction_settings.denominator_range.min == 0 {
                return ErrorCode::ZeroDenominator.into();
            }

            let instance = chinese_format_generator.clone();
            generator_functions.push(Box::new(move || {
                instance
                    .fraction(
                        (&fraction_settings.denominator_range).into(),
                        (&fraction_settings.numerator_range).into(),
                    )
                    .expect("Denominator is zero by construction")
                    .to_chinese(settings.variant.into())
            }))
        }

        Ok(Self {
            generator_functions,
        })
    }

    #[wasm_bindgen]
    pub fn logograms(&self) -> String {
        let generator_function = fastrand::choice(self.generator_functions.iter())
            .expect("There is always at least a function");

        let chinese = generator_function();

        chinese.logograms
    }
}
