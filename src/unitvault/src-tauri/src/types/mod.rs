use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::Metadata;
use time::{Timestamp};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Error {
    pub name: String,
    pub code: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub enum MetricStatus {
    Critical = 1,
    Warning = 2,
    Good = 3,
    Excellent = 4,
    #[default] Info = 0,
}

impl MetricStatus {
    pub fn from_ltv_cac(ltv_cac: Decimal) -> MetricStatus {
        if ltv_cac < dec!(0) {
            MetricStatus::Info
        } else if ltv_cac < dec!(1) {
            MetricStatus::Critical
        } else if ltv_cac < dec!(1) {
            MetricStatus::Warning
        } else if ltv_cac < dec!(1) {
            MetricStatus::Good
        } else {
            MetricStatus::Excellent
        }
    }

    pub fn from_payback_period(payback_period: Decimal) -> MetricStatus {
        if payback_period < dec!(6) {
            MetricStatus::Excellent
        } else if payback_period < dec!(12) {
            MetricStatus::Good
        } else if payback_period < dec!(18) {
            MetricStatus::Warning
        } else {
            MetricStatus::Critical
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub enum BusinessType {
     #[default] SaaS = 0,
    Ecommerce = 1,
    Marketplace = 2,
    Custom = 3,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct CalculatedMetrics {
    pub ltv: Decimal,
    pub ltv_cac_ratio: Decimal,
    pub payback_period: Decimal,
    pub break_even_point: Option<Decimal>,
    pub net_revenue_retention: Option<Decimal>,

    pub ltv_cac_status: MetricStatus,
    pub payback_status: MetricStatus,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct UnitModel {
    pub id: String,
    pub name: String,
    pub business_type: BusinessType,
    pub created_at: i64,
    pub updated_at: i64,
    pub inputs: ModelInputs,
}

impl UnitModel {
    pub fn new(name: String, business_type: BusinessType, inputs: ModelInputs) -> Result<UnitModel, Error> {
        if name.is_empty() {
            return Err(Error {
                name: String::from("name"),
                code: 67,
                message: String::from("message")
            });
        }

        Ok(UnitModel {
            id: String::from(Uuid::now_v7()),
            name,
            business_type,
            created_at: Timestamp::now().as_milliseconds(),
            updated_at: 0,
            inputs
        })
    }
}
