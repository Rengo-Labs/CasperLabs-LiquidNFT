use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    InvalidOwner = 65,
    InvalidAddress,
    NotContributionPhase,
    InvalidIncrease,
    InvalidDecrease,
    ProviderExists,
    BelowFloor,
    EnabledLocker,
    FloorReached,
    InvalidTrustee,
    NotEnoughTime,
    AlreadyStarted,
    InvalidSender,
    TooLate,
    MinimumPayoff,
    TooEarly,
    NotSingleProvider,
    SingleProviderExists,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
