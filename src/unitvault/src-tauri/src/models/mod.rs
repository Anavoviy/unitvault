
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::Timestamp;
use uuid::Uuid;

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub enum MetricStatus {
    Info = 0,
    Great = 1,
    Good = 2,
    Warning = 3,
    Critical = 4
}

#[derive(PartialEq, Deserialize, Serialize, Debug, Default, Clone)]
pub enum BusinessType {
    #[default]
    SaaS = 0,
    Ecommerce = 1,
    Marketplace = 2,
    Custom = 3
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct InputMetrics {
    // Базовые метрики
    pub cac: Decimal,
    pub margin: Decimal,
    pub time_horizon: u8,

    // Опциональные метрики
    pub arpu: Option<Decimal>, // SaaS & E-commerce
    pub churn_rate: Option<Decimal>, // SaaS & E-commerce
    pub aov: Option<Decimal>, // E-commerce & Marketplace
    pub purchase_frequency: Option<Decimal>, // E-commerce & Marketplace

    pub fixed_cost: Option<Decimal>, // Universal
    pub expansion_revenue: Option<Decimal>, // SaaS & Marketplace
    pub take_rate: Option<Decimal>, // Marketplace
}
pub struct MarketplaceInputs {
    pub cac: Decimal,
    pub margin: Decimal,
    pub time_horizon: u8,

    pub aov: Decimal,
    pub purchase_frequency: Decimal,
    pub take_rate: Decimal,

    pub fixed_cost: Option<Decimal>,
    pub expansion_revenue: Option<Decimal>
}
pub struct SaaSInputs {
    pub cac: Decimal,
    pub margin: Decimal,
    pub time_horizon: u8,

    pub arpu: Decimal,
    pub churn_rate: Decimal,

    pub fixed_cost: Option<Decimal>,
    pub expansion_revenue: Option<Decimal>,
}
pub struct EcomInputs {
    pub cac: Decimal,
    pub margin: Decimal,
    pub time_horizon: u8,

    pub arpu: Decimal,
    pub churn_rate: Decimal,
    pub aov: Decimal,
    pub purchase_frequency: Decimal,

    pub fixed_cost: Option<Decimal>,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CalculateMetrics {
    pub ltv: Decimal,
    pub ltv_cac: Decimal,
    pub payback_period: Decimal,
    pub break_even_point: Decimal,
    pub nrr: Option<Decimal>, // SaaS

    // Графики
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UnitModel {
    pub id: String,
    pub name: String,
    pub business_type: BusinessType,

    pub inputs: InputMetrics,
    pub calculated: CalculateMetrics,

    pub created_at: u64,
    pub updated_at: u64
}

impl UnitModel {
    pub fn create_saas(name: String, inputs: SaaSInputs) -> UnitModel {
        UnitModel {
            id: String::from(Uuid::now_v7()),
            name,
            business_type: BusinessType::SaaS,

            inputs: InputMetrics {
                cac: inputs.cac,
                margin: inputs.margin,
                time_horizon: if inputs.time_horizon == 0 { 24 } else { inputs.time_horizon },
                arpu: None,
                churn_rate: None,
                aov: None,
                purchase_frequency: None,
                fixed_cost: None,
                expansion_revenue: None,
                take_rate: None,
            },

            calculated: Default::default(),
            created_at: Timestamp::now().as_milliseconds() as u64,
            updated_at: 0,
        }
    }

    pub fn create_ecom(name: String, inputs: EcomInputs) -> UnitModel {
        UnitModel {
            id: String::from(Uuid::now_v7()),
            name,
            business_type: BusinessType::SaaS,

            inputs : InputMetrics {
                cac: inputs.cac,
                margin: inputs.margin,
                time_horizon: if inputs.time_horizon == 0 { 24 } else { inputs.time_horizon },

                arpu: Some(inputs.arpu),
                churn_rate: Some(inputs.churn_rate),
                aov: Some(inputs.aov),
                purchase_frequency: Some(inputs.purchase_frequency),

                fixed_cost: inputs.fixed_cost,
                expansion_revenue: None,
                take_rate: None,
            },

            calculated: Default::default(),
            created_at: Timestamp::now().as_milliseconds() as u64,
            updated_at: 0,
        }
    }

    pub fn create_marketplace(name: String, inputs: MarketplaceInputs) -> UnitModel {
        UnitModel {
            id: String::from(Uuid::now_v7()),
            name,
            business_type: BusinessType::SaaS,

            inputs : InputMetrics {
                cac: inputs.cac,
                margin: inputs.margin,
                time_horizon: if inputs.time_horizon == 0 { 24 } else { inputs.time_horizon },

                arpu: None,
                churn_rate: None,
                aov: Some(inputs.aov),
                purchase_frequency: Some(inputs.purchase_frequency),
                take_rate: Some(inputs.take_rate),

                fixed_cost: inputs.fixed_cost,
                expansion_revenue: inputs.expansion_revenue
            },

            calculated: Default::default(),
            created_at: Timestamp::now().as_milliseconds() as u64,
            updated_at: 0,
        }
    }
}