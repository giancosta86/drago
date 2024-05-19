use drago::*;
use pretty_assertions::assert_eq as eq;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_integer() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: Some(SignedRange(92, 92)),
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "九十二".to_string());
}

#[wasm_bindgen_test]
fn test_fraction() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: Some(FractionParams {
            denominatorRange: UnsignedRange(7, 7),
            numeratorRange: SignedRange(3, 3),
        }),
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "七分之三".to_string());
}

#[wasm_bindgen_test]
fn test_count() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: Some(UnsignedRange(2, 2)),
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "两".to_string());
}

#[wasm_bindgen_test]
fn test_count_traditional() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Traditional,
        integerRange: None,
        fraction: None,
        countRange: Some(UnsignedRange(2, 2)),
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "兩".to_string());
}

#[wasm_bindgen_test]
fn test_digit_sequence() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: Some(UnsignedRange(5, 5)),
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "七二四二六".to_string());
}

#[wasm_bindgen_test]
fn test_zero_len_digit_sequence() {
    let creation_result = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: Some(UnsignedRange(0, 5)),
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    });

    eq!(
        creation_result.unwrap_err(),
        RandomParamsError::DigitSequenceLength(UnsignedRangeError::InvalidStart(0))
    );
}

#[wasm_bindgen_test]
fn test_decimal() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: Some(DecimalParams {
            integerRange: SignedRange(42, 42),
            fractionalLengthRange: UnsignedRange(3, 3),
        }),
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "四十二点四二六".to_string());
}

#[wasm_bindgen_test]
fn test_renminbi() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: Some(RenminbiParams {
            style: CurrencyStyle::Formal,
            yuanRange: UnsignedRange(7, 7),
            includeDimes: true,
            includeCents: true,
        }),
        date: None,
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "七元七角两分".to_string());
}

#[wasm_bindgen_test]
fn test_date() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: Some(DateParams {
            formal: true,
            pattern: DatePattern::YearMonthDayWeekDay,
            yearRange: Some(UnsignedRange(2019, 2019)),
            weekFormat: Some(WeekFormat::XinqQi),
        }),
        linearTime: None,
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "二零一九年九月九号星期三".to_string());
}

#[wasm_bindgen_test]
fn test_linear_time() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: Some(LinearTimeParams {
            dayPart: true,
            includeSecond: true,
        }),
        deltaTime: false,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "上午八点四十三分十七秒".to_string());
}

#[wasm_bindgen_test]
fn test_delta_time() {
    let logogram_generator = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: true,
    })
    .unwrap();

    let logograms = logogram_generator.logograms();

    eq!(logograms, "六点差十七分".to_string());
}

#[wasm_bindgen_test]
fn test_no_params() {
    let creation_result = LogogramGenerator::create(RandomParams {
        seed: 90,
        variant: Variant::Simplified,
        integerRange: None,
        fraction: None,
        countRange: None,
        digitSequenceLengthRange: None,
        decimal: None,
        renminbi: None,
        date: None,
        linearTime: None,
        deltaTime: false,
    });

    eq!(
        creation_result.unwrap_err(),
        RandomParamsError::EmptyRandomParams
    );
}
