use crate::dto;

// TODO: ensure that negative numbers trigger an error - otherwise, you'll have to check them yourself
dto! {
    pub struct UnsignedRangeDto(pub u128, pub u128);
}

dto! {
    pub struct SignedRangeDto (pub i128, pub i128);
}
