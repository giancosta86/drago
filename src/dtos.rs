use std::ops::RangeInclusive;

macro_rules! webify {
    ($declaration:item) => {
        #[derive(tsify::Tsify, serde::Serialize, serde::Deserialize, Debug)]
        #[tsify(into_wasm_abi, from_wasm_abi)]
        $declaration
    };
}

webify! {
    pub struct GeneratorSettings {
        pub use_simplified_characters: bool,

        pub enable_approximations: bool,

        pub cardinal_settings: Option<CardinalSettings>,

        pub sequence_settings: Option<SequenceSettings>,

        pub float_settings: Option<FloatSettings>,

        pub fraction_settings: Option<FractionSettings>,

        pub date_settings: Option<DateSettings>,

        pub time_settings: Option<TimeSettings>,

        pub enable_ages: bool,

        pub enable_ordinals: bool,

        pub enable_money: bool
    }
}

webify! {
    pub struct CardinalSettings {
        pub range: RangeInclusive<u32>,
    }
}

webify! {
    pub struct SequenceSettings {
        pub length_range: RangeInclusive<u8>
    }
}

webify! {
    pub struct FloatSettings {
        pub integer_part_settings: CardinalSettings,
        pub decimal_part_settings: SequenceSettings,
    }
}

webify! {
    pub struct FractionSettings {
        pub max_denominator: u32,
    }
}

webify! {
    pub struct DateSettings {
        pub include_year: bool,
    }
}

webify! {
    pub struct TimeSettings {
        pub include_day_part: bool,
        pub include_seconds: bool,
        pub include_half_quarter: bool,
        pub include_to: bool
    }
}
