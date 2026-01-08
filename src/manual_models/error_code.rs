use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum ErrorCode {
    #[default]
    OrderNotFound = 1,
    DuplicateOrderId = 2,
    TooManyPendingRequests = 3,
    ThrottleExceeded = 4,
    UnknownInstrument = 5,
    InvalidRequest = 6,
    NotLoggedIn = 7,
    AlreadySubscribed = 8,
    NotSubscribed = 9,
    PartitionUnavailable = 10,
    SessionExhausted = 11,
    InvalidOrderId = 12,
    InvalidPriceOrNonPositivePrice = 13, // combines InvalidPrice | NonPositivePrice
    InvalidQuantityOrAmountNotAlignedWithTick = 14, // combines InvalidQuantity | AmountNotAlignedWithTick
    PriceNotAlignedWithTick = 15,
    InstrumentExists = 16,
    UnknownProduct = 17,
    ProductExists = 18,
    AuthenticationFailed = 19,
    TemporaryFailure = 20,
    InvalidPartition = 21,
    InvalidProtGroup = 22,
    ProtNotSet = 23,
    InsufficientMargin = 24, // contains data in original { .. }
    InvalidTimeInForce = 25,
    AccountLocked = 26,
    InsufficientFunds = 27,
    Unauthorized = 28,
    NeedTOTP = 29,
    NeedSMS = 30,
    UnknownMethod = 31,
    Require2FA = 32,
    PasswordNotAccepted = 33,
    InvalidVerificationCode = 34,
    InvalidEmailAddress = 35,
    DeprecatedInvalidSessionForMassQuoting = 36,
    InvalidState = 37,
    InvalidPhoneNumber = 38,
    UnknownAsset = 39,
    UnknownUnderlying = 40,
    InsufficientLiquidity = 41,
    ExcludedJurisdiction = 42,
    PriceCollarBreach = 43,
    QuoteExceedsProtGroupSize = 44, // contains data in original { .. }
    UnboundedMarketOrder = 45,
    UnsupportedBtcTransfer = 46,
    SMSThrottleExceeded = 47,
    InsufficientFeeFunds = 48,
    TooManyOpenOrders = 49,
    TotalOrderSizeTooLarge = 50,
    MqpNotEnabled = 51,
    PersistentSession = 52,
    ApiConnectionLimitReached = 53, // contains data in original { .. }
    AssetNotTransactable = 54,
    MaxBotsReached = 55, // contains data in original { .. }
    ComboTotalGrossAmountTooLarge = 56,
    ReduceOnlyMode = 57,
    RejectedOnVolumeQuota = 58,
}
