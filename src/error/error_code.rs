use crate::dto;
use std::fmt::Display;

dto! {
    pub enum ErrorCode {
        ZeroDenominator
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDenominator => write!(f, "The denominator is zero"),
        }
    }
}
