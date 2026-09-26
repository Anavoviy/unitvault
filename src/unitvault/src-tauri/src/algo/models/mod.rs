use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// SaaS или подписочная модель
pub struct SaaSMetrics {
    pub arpu: Decimal,
    pub margin: Decimal,
    pub churn_rate: Decimal,
}

// E-commerce
pub struct EcomMetrics {
    pub aov: Decimal,
    pub purchase_frequency: Decimal,
    pub margin: Decimal,
    pub customer_lifespan: Decimal,
}

// LTV/CAC
pub struct UnitEconomics {
    pub ltv: Decimal,
    pub cac: Decimal,
}

// Payback period
pub struct PaybackData {
    pub cac: Decimal,
    pub arpu: Decimal,
    pub margin: Decimal,
}

// Break Even
pub struct BreakEvenData {
    pub fixed_costs: Decimal,
    pub arpu: Decimal,
    pub margin: Decimal,
    pub cac: Decimal,
    pub churn_rate: Decimal,
}
