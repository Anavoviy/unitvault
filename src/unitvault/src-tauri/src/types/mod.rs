use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::Metadata;

#[derive(Debug)]
pub struct Error {
    pub name: String,
    pub code: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MetricStatus {
    Critical,
    Warning,
    Good,
    Excellent,
    Info,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BusinessType {
    SaaS,
    Ecommerce,
    Marketplace,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ModelInputs {
    // Базовые
    pub cac: Decimal,
    pub arpu: Decimal,
    pub margin: Decimal,     // 0.0 - 1.0
    pub churn_rate: Decimal, // 0.0 - 1.0

    // Опциональные
    pub time_horizon: u32, // default = 24 мес.
    pub fixed_costs: Option<Decimal>,
    pub expansion_revenue: Option<Decimal>,
    pub discount_rate: Option<Decimal>,
    pub aov: Option<Decimal>,
    pub purchase_frequency: Option<Decimal>,
    pub take_rate: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CalculatedMetrics {
    pub ltv: Decimal,
    pub ltv_cac_ratio: Decimal,
    pub payback_period: Decimal,
    pub break_even_point: Option<Decimal>,
    pub net_revenue_retention: Option<Decimal>,

    pub ltv_cac_status: MetricStatus,
    pub payback_status: MetricStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UnitModel {
    pub id: String,
    pub name: String,
    pub business_type: BusinessType,
    pub created_at: i64,
    pub updated_at: i64,
    pub inputs: ModelInputs,
}
