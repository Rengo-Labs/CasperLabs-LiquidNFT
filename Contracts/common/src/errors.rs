use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    LiquidHelperUnderflowSub0 = 100,
    LiquidHelperUnderflowSub1 = 101,
    LiquidLockerUnderflowSub0 = 102,
    LiquidLockerUnderflowSub1 = 103,
    LiquidLockerUnderflowSub2 = 104,
    LiquidLockerUnderflowSub3 = 105,
    LiquidLockerUnderflowSub4 = 106,
    LiquidLockerUnderflowSub5 = 107,
    LiquidLockerUnderflowSub6 = 108,
    LiquidLockerUnderflowSub7 = 109,
    LiquidLockerUnderflowSub8 = 110,
    LiquidLockerUnderflowSub9 = 111,
    LiquidLockerUnderflowSub10 = 112,
    LiquidLockerUnderflowSub11 = 113,
    LiquidLockerUnderflowSub12 = 114,
    LiquidLockerDivision0 = 115,
    LiquidLockerDivision1 = 116,
    LiquidLockerDivision2 = 117,
    LiquidLockerDivision3 = 118,
    LiquidLockerDivision4 = 119,
    LiquidLockerDivision5 = 120,
    LiquidLockerDivision6 = 121,
    LiquidLockerDivision7 = 122,
    LiquidLockerDivision8 = 123,
    LiquidLockerDivision9 = 124,
    LiquidLockerDivision10 = 125,
    InvalidMaster = 126,
    InvalidLocker = 127,
    InvalidOwner = 128,
    InvalidAddress = 129,
    NotContributionPhase = 130,
    InvalidIncrease = 131,
    InvalidDecrease = 132,
    ProviderExists = 133,
    BelowFloor = 134,
    EnabledLocker = 135,
    FloorReached = 136,
    InvalidTrustee = 137,
    NotEnoughTime = 138,
    AlreadyStarted = 139,
    InvalidSender = 140,
    TooLate = 141,
    MinimumPayoff = 142,
    TooEarly = 143,
    NotSingleProvider = 144,
    SingleProviderExists = 145,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
