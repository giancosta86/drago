use crate::dto;
use chinese_rand::gregorian;

dto! {
    pub struct DeltaTimeParams {
        pub formal: bool,
    }
}

impl From<DeltaTimeParams> for gregorian::DeltaTimeParams {
    fn from(dto: DeltaTimeParams) -> Self {
        Self { formal: dto.formal }
    }
}
