mod decimal;
mod fraction;

use chinese_format::{CountBase, Variant};
use chinese_rand::{
    gregorian::{DateParams, LinearTimeParams},
    RenminbiParams,
};
use std::ops::RangeInclusive;

pub use decimal::*;
pub use fraction::*;

pub type DigitSequenceLength = u8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RandomParams {
    pub seed: u64,
    pub variant: Variant,
    pub integer_range: Option<RangeInclusive<i128>>,
    pub fraction: Option<FractionParams>,
    pub count_range: Option<RangeInclusive<CountBase>>,
    pub digit_sequence_length_range: Option<RangeInclusive<DigitSequenceLength>>,
    pub decimal: Option<DecimalParams>,
    pub renminbi: Option<RenminbiParams>,
    pub date: Option<DateParams>,
    pub linear_time: Option<LinearTimeParams>,
    pub delta_time: bool,
}
