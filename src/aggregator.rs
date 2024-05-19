use chinese_format::ChineseFormat;
use chinese_rand::ChineseFormatGenerator;
use std::rc::Rc;

type RandomFunction = dyn Fn() -> Box<dyn ChineseFormat>;

pub struct RandomFunctionAggregator {
    random_functions: Vec<Box<RandomFunction>>,
    chinese_format_generator: Rc<ChineseFormatGenerator>,
}

impl RandomFunctionAggregator {
    pub fn new(chinese_format_generator: ChineseFormatGenerator) -> Self {
        Self {
            random_functions: vec![],
            chinese_format_generator: Rc::new(chinese_format_generator),
        }
    }

    pub fn try_to_register<
        TParams: 'static,
        TResult: ChineseFormat + 'static,
        TFunctionBody: Fn(&ChineseFormatGenerator, &TParams) -> TResult + 'static,
    >(
        &mut self,
        function_params: Option<TParams>,
        function_body: TFunctionBody,
    ) {
        if let Some(body_params) = function_params {
            let dedicated_generator_reference = self.chinese_format_generator.clone();

            self.random_functions.push(Box::new(move || {
                let chinese_format = function_body(&dedicated_generator_reference, &body_params);

                Box::new(chinese_format)
            }))
        }
    }

    pub fn len(&self) -> usize {
        self.random_functions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.random_functions.is_empty()
    }

    pub fn generate(&self) -> Box<dyn ChineseFormat> {
        let generator_function = fastrand::choice(self.random_functions.iter())
            .expect("There is always at least a function, by construction");

        generator_function()
    }
}
