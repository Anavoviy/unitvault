pub enum ModelError {
    ArpuNotFound,
    ChurnRateNotFound,
    AovNotFound,
    PurchaseFrequencyNotFound,
    TakeRateNotFound,
    FixedCostNotFound,

    BusinessTypeNotSet,

    CacParseInvalid,
    MarginParseInvalid,
    TimeHorizonParseInvalid,
    ArpuParseInvalid,
    ChurnRateParseInvalid,
    AovParseInvalid,
    PurchaseFrequencyParseInvalid,
    FixedCostParseInvalid,
    ExpansionRevenueParseInvalid,
    TakeRateParseInvalid,
}