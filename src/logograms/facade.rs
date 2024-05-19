use super::logic;
use crate::{
    dtos::{RandomParams, RandomParamsError},
    params,
    utils::set_panic_hook,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LogogramGenerator {
    logic: logic::LogogramGenerator,
}

#[wasm_bindgen]
impl LogogramGenerator {
    pub fn create(randomParams: RandomParams) -> Result<LogogramGenerator, RandomParamsError> {
        set_panic_hook();

        let random_params: params::RandomParams = randomParams.try_into()?;

        Ok(Self {
            logic: logic::LogogramGenerator::try_new(random_params)?,
        })
    }

    pub fn logograms(&self) -> String {
        self.logic.logograms()
    }
}

impl std::fmt::Debug for LogogramGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.logic)
    }
}
