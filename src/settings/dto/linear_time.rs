use crate::dto;

dto! {
    pub struct LinearTimeSettingsDto {
        pub dayPart: bool,
        pub includeSecond: bool,
    }
}
