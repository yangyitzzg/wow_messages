/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_stable_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_stable_result.wowm#L1):
/// ```text
/// enum StableResult : u8 {
///     ERR_MONEY = 0x01;
///     ERR_STABLE = 0x06;
///     SUCCESS_STABLE = 0x08;
///     SUCCESS_UNSTABLE = 0x09;
///     SUCCESS_BUY_SLOT = 0x0A;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StableResult {
    /// you don't have enough money
    ErrMoney,
    /// currently used in most fail cases
    ErrStable,
    /// table success
    SuccessStable,
    /// unstable/swap success
    SuccessUnstable,
    /// buy slot success
    SuccessBuySlot,
}

impl StableResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::ErrMoney => 0x1,
            Self::ErrStable => 0x6,
            Self::SuccessStable => 0x8,
            Self::SuccessUnstable => 0x9,
            Self::SuccessBuySlot => 0xa,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::ErrMoney,
            Self::ErrStable,
            Self::SuccessStable,
            Self::SuccessUnstable,
            Self::SuccessBuySlot,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl StableResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::ErrMoney => "ERR_MONEY",
            Self::ErrStable => "ERR_STABLE",
            Self::SuccessStable => "SUCCESS_STABLE",
            Self::SuccessUnstable => "SUCCESS_UNSTABLE",
            Self::SuccessBuySlot => "SUCCESS_BUY_SLOT",
        }
    }

}

const NAME: &str = "StableResult";

impl Default for StableResult {
    fn default() -> Self {
        Self::ErrMoney
    }
}

impl std::fmt::Display for StableResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrMoney => f.write_str("ErrMoney"),
            Self::ErrStable => f.write_str("ErrStable"),
            Self::SuccessStable => f.write_str("SuccessStable"),
            Self::SuccessUnstable => f.write_str("SuccessUnstable"),
            Self::SuccessBuySlot => f.write_str("SuccessBuySlot"),
        }
    }
}

impl TryFrom<u8> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ErrMoney),
            6 => Ok(Self::ErrStable),
            8 => Ok(Self::SuccessStable),
            9 => Ok(Self::SuccessUnstable),
            10 => Ok(Self::SuccessBuySlot),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

