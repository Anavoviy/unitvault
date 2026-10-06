pub mod errors;
pub mod models;

use std::sync::Mutex;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tauri::State;
use crate::AppState;
use self::errors::{
    break_even_denominator_0, ltv_churnrate_lower_0, ltv_ecommerce_parts_lower_0,
    ltvcac_cac_lower_0, payback_arpu_or_margin_lower_0,
};
use self::models::{BreakEvenData, EcomMetrics, PaybackData, SaaSMetrics, UnitEconomics};
use crate::types::{CalculatedMetrics, Error, MetricStatus, UnitModel};

pub fn ltv(metrics: &SaaSMetrics) -> Result<Decimal, Error> {
    if metrics.churn_rate <= dec!(0) {
        return Err(ltv_churnrate_lower_0());
    }

    Ok((metrics.arpu * metrics.margin) / metrics.churn_rate)
}
pub fn ltv_e_com(metrics: &EcomMetrics) -> Result<Decimal, Error> {
    let ltv: Decimal =
        metrics.aov * metrics.purchase_frequency * metrics.margin * metrics.customer_lifespan;

    if ltv <= dec!(0) {
        return Err(ltv_ecommerce_parts_lower_0());
    }

    Ok(ltv)
}

pub fn ltv_cac(metrics: &UnitEconomics) -> Result<Decimal, Error> {
    if metrics.cac <= dec!(0) {
        return Err(ltvcac_cac_lower_0());
    }

    Ok(metrics.ltv / metrics.cac)
}

pub fn payback_period(metrics: &PaybackData) -> Result<Decimal, Error> {
    if metrics.arpu <= dec!(0) || metrics.margin <= dec!(0) {
        return Err(payback_arpu_or_margin_lower_0());
    }

    Ok(metrics.cac / (metrics.arpu * metrics.margin))
}

pub fn break_even(metrics: &BreakEvenData) -> Result<Decimal, Error> {
    let left = metrics.arpu * metrics.margin;
    let right = metrics.cac * metrics.churn_rate;
    if left < dec!(0) || right <= dec!(0) || left <= right {
        return Err(break_even_denominator_0());
    }

    Ok((metrics.fixed_costs / (left - right)).ceil())
}

// Statistics
pub fn retention_curve(model: &UnitModel) -> Result<Vec<Decimal>, Error> {
    if model.inputs.churn_rate < dec!(0) || model.inputs.churn_rate >= dec!(1) {
        return Err(ltv_churnrate_lower_0());
    }

    let one_minus_churn = dec!(1) - model.inputs.churn_rate;
    let mut statistics = Vec::new();

    let mut current = dec!(1);

    for i in 0..=model.inputs.time_horizon {
        statistics.push(current);
        current *= one_minus_churn;
    }

    Ok(statistics)
}



// Pub endpoints
pub fn calculate_metrics(unit: &UnitModel) -> Result<CalculatedMetrics, Error> {
    let ltv = ltv(&SaaSMetrics {
        arpu: unit.inputs.cac,
        margin: unit.inputs.margin,
        churn_rate: unit.inputs.churn_rate
    }).unwrap();

    let ltv_cac = ltv_cac(&UnitEconomics {
        ltv,
        cac: unit.inputs.cac,
    }).unwrap();
    let payback_period = payback_period(&PaybackData {
        cac: unit.inputs.cac,
        arpu: unit.inputs.arpu,
        margin: unit.inputs.margin
    }).unwrap();

    let calculated = CalculatedMetrics {
        ltv,
        ltv_cac_ratio: ltv_cac,
        payback_period,
        break_even_point: Some(break_even(&BreakEvenData {
            fixed_costs: unit.inputs.fixed_costs.unwrap_or(dec!(0)),
            arpu: unit.inputs.arpu,
            margin: unit.inputs.margin,
            cac: unit.inputs.cac,
            churn_rate: unit.inputs.churn_rate
        }).unwrap_or(dec!(0))),
        net_revenue_retention: None,
        ltv_cac_status: MetricStatus::from_ltv_cac(ltv_cac),
        payback_status: MetricStatus::from_payback_period(payback_period),
    };

    Ok(calculated)
}

