use crate::{aggregator::RandomFunctionAggregator, dtos::RandomParamsError, params::RandomParams};
use chinese_format::Variant;
use chinese_rand::{ChineseFormatGenerator, FastRandGenerator};

pub struct LogogramGenerator {
    aggregator: RandomFunctionAggregator,
    variant: Variant,
}

impl LogogramGenerator {
    pub fn try_new(random_params: RandomParams) -> Result<Self, RandomParamsError> {
        fastrand::seed(random_params.seed);

        let mut aggregator =
            RandomFunctionAggregator::new(ChineseFormatGenerator::new(FastRandGenerator::new()));

        aggregator.try_to_register(random_params.integer_range, |generator, integer_range| {
            generator.integer(integer_range.clone())
        });

        aggregator.try_to_register(random_params.fraction, |generator, fraction_params| {
            generator
                .fraction(
                    fraction_params.denominator_range.clone(),
                    fraction_params.numerator_range.clone(),
                )
                .expect("Denominator is zero by construction")
        });

        aggregator.try_to_register(random_params.count_range, |generator, count_range| {
            generator.count(count_range.clone())
        });

        aggregator.try_to_register(
            random_params.digit_sequence_length_range,
            |generator, digit_sequence_length_range| {
                generator.digit_sequence(digit_sequence_length_range.clone())
            },
        );

        aggregator.try_to_register(random_params.decimal, |generator, decimal_params| {
            generator.decimal(
                decimal_params.integer_range.clone(),
                decimal_params.fractional_length_range.clone(),
            )
        });

        aggregator.try_to_register(random_params.renminbi, |generator, renminbi_params| {
            generator.renminbi(renminbi_params.clone())
        });

        aggregator.try_to_register(random_params.date, |generator, date_params| {
            generator.gregorian().date(date_params.clone())
        });

        aggregator.try_to_register(
            random_params.linear_time,
            |generator, linear_time_params| generator.gregorian().linear_time(*linear_time_params),
        );

        if random_params.delta_time {
            aggregator.try_to_register(Some(()), |generator, _| generator.gregorian().delta_time());
        }

        if aggregator.is_empty() {
            return Err(RandomParamsError::EmptyRandomParams);
        }

        Ok(Self {
            aggregator,
            variant: random_params.variant,
        })
    }

    pub fn logograms(&self) -> String {
        let chinese_format = self.aggregator.generate();

        let chinese = chinese_format.to_chinese(self.variant);

        chinese.logograms
    }
}

impl std::fmt::Debug for LogogramGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Logogram generator ({} random functions)",
            self.aggregator.len()
        )
    }
}
