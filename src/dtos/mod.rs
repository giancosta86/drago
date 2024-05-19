mod currency;
mod date;
mod decimal;
mod error;
mod fraction;
mod linear_time;
mod ranges;
mod variant;

use crate::{
    dto,
    params::{self, DigitSequenceLength},
};
use chinese_rand::gregorian;
use std::ops::RangeInclusive;

pub use currency::*;
pub use date::*;
pub use decimal::*;
pub use error::*;
pub use fraction::*;
pub use linear_time::*;
pub use ranges::*;
pub use variant::*;

dto! {
    pub struct RandomParams {
        pub seed: u64,
        pub variant: Variant,
        #[tsify(optional)]
        pub integerRange: Option<SignedRange>,
        #[tsify(optional)]
        pub fraction: Option<FractionParams>,
        #[tsify(optional)]
        pub countRange: Option<UnsignedRange>,
        #[tsify(optional)]
        pub digitSequenceLengthRange: Option<UnsignedRange>,
        #[tsify(optional)]
        pub decimal: Option<DecimalParams>,
        #[tsify(optional)]
        pub renminbi: Option<RenminbiParams>,
        #[tsify(optional)]
        pub date: Option<DateParams>,
        #[tsify(optional)]
        pub linearTime: Option<LinearTimeParams>,
        pub deltaTime: bool
    }
}

impl TryFrom<RandomParams> for params::RandomParams {
    type Error = RandomParamsError;

    fn try_from(dto: RandomParams) -> Result<Self, Self::Error> {
        let integer_range = dto
            .integerRange
            .map(|dto| dto.try_into().map_err(RandomParamsError::Integer))
            .transpose()?;

        let fraction_params = dto
            .fraction
            .map(|dto| dto.try_into().map_err(RandomParamsError::Fraction))
            .transpose()?;

        let count_range = dto
            .countRange
            .map(|dto| dto.try_into().map_err(RandomParamsError::Count))
            .transpose()?;

        let digit_sequence_length_range = dto
            .digitSequenceLengthRange
            .map(|dto| {
                let range: RangeInclusive<DigitSequenceLength> = dto
                    .try_into()
                    .map_err(RandomParamsError::DigitSequenceLength)?;

                if *range.start() == 0 {
                    return Err(RandomParamsError::DigitSequenceLength(
                        UnsignedRangeError::InvalidStart(0),
                    ));
                }

                Ok(range)
            })
            .transpose()?;

        let decimal_params = dto
            .decimal
            .map(|dto| dto.try_into().map_err(RandomParamsError::Decimal))
            .transpose()?;

        let renminbi_params = dto
            .renminbi
            .map(|dto| dto.try_into().map_err(RandomParamsError::Renminbi))
            .transpose()?;

        let date_params = dto
            .date
            .map(|dto| dto.try_into().map_err(RandomParamsError::Date))
            .transpose()?;

        let linear_time_params: Option<gregorian::LinearTimeParams> =
            dto.linearTime.map(|dto| dto.into());

        Ok(Self {
            seed: dto.seed,
            variant: dto.variant.into(),
            integer_range,
            fraction: fraction_params,
            count_range,
            digit_sequence_length_range,
            decimal: decimal_params,
            renminbi: renminbi_params,
            date: date_params,
            linear_time: linear_time_params,
            delta_time: dto.deltaTime,
        })
    }
}
