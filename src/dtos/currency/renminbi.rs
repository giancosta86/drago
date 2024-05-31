use crate::{settings::RenminbiSettings, CurrencyStyleDto, RenminbiError, SignedRangeDto};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenminbiSettingsDto {
    pub style: CurrencyStyleDto,
    pub yuan_range: SignedRangeDto,
    pub include_dimes: bool,
    pub include_cents: bool,
}

impl TryFrom<RenminbiSettingsDto> for RenminbiSettings {
    type Error = RenminbiError;

    fn try_from(dto: RenminbiSettingsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            style: dto.style.into(),
            yuan_range: dto.yuan_range.try_into().map_err(RenminbiError::Yuan)?,
            include_dimes: dto.include_dimes,
            include_cents: dto.include_cents,
        })
    }
}
