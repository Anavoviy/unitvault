use crate::algo::models::{BreakEvenData, EcomMetrics, PaybackData, SaaSMetrics, UnitEconomics};
use crate::algo::{break_even, ltv, ltv_cac, ltv_e_com, payback_period};
use crate::crypto::encryption::EncryptedData;
use crate::crypto::obfuscation::{deobfuscate, obfuscate};
use crate::local::{load, save};
use crate::types::{BusinessType, ModelInputs, UnitModel};
use rmp_serde::{Deserializer, Serializer};
#[cfg(test)]
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::Serialize;
use std::fs::{File, Metadata};
use std::io::{Read, Write};
use uuid::Uuid;

//LTV
#[test]
pub fn test_ltv_with_zero_churn_rate() {
    let metrics = SaaSMetrics {
        arpu: dec!(350),
        margin: dec!(0.65),
        churn_rate: dec!(0),
    };

    let result = ltv(&metrics).unwrap_err();
    assert_eq!(result.code, 11);
    assert_eq!(result.name, "InvalidChurnRate");
}

#[test]
pub fn ltv_correct() {
    let metrics = SaaSMetrics {
        arpu: dec!(350),
        margin: dec!(0.65),
        churn_rate: dec!(0.15),
    };

    // 350.0 * 0.65 / 0.15 = 1516.666...
    let result = ltv(&metrics).unwrap().round_dp(2);
    assert_eq!(result, dec!(1516.67));
}

// LTV e-commerce
#[test]
pub fn ltv_e_comm_with_customerlifespan_0() {
    let metrics = EcomMetrics {
        aov: dec!(1500),
        purchase_frequency: dec!(6.3),
        margin: dec!(0.04),
        customer_lifespan: dec!(0),
    };

    let result = ltv_e_com(&metrics).unwrap_err();
    assert_eq!(result.code, 12);
    assert_eq!(result.name, String::from("InvalidParametersLtvECommerce"));
}

#[test]
pub fn ltv_e_comm_correct() {
    let metrics = EcomMetrics {
        aov: dec!(1500),
        purchase_frequency: dec!(6.3),
        margin: dec!(0.04),
        customer_lifespan: dec!(8.5),
    };

    // 1500 * 6.3 * 0.04 * 8.5 = 3213
    let result = ltv_e_com(&metrics).unwrap();
    assert_eq!(result, dec!(3213));
}

// LTV/CAC
#[test]
pub fn ltv_cac_with_cac_0() {
    let metrics = UnitEconomics {
        ltv: dec!(1500),
        cac: dec!(0),
    };

    let result = ltv_cac(&metrics).unwrap_err();
    assert_eq!(result.code, 13);
    assert_eq!(result.name, String::from("InvalidCac"));
}

#[test]
pub fn ltv_cac_correct() {
    let metrics = UnitEconomics {
        ltv: dec!(1500),
        cac: dec!(30),
    };

    // 1500 / 30 = 50
    let result = ltv_cac(&metrics).unwrap();
    assert_eq!(result, dec!(50));
}

// Payback Pediod
#[test]
pub fn payback_period_margin_0() {
    let metrics = PaybackData {
        cac: dec!(30),
        arpu: dec!(650),
        margin: dec!(0),
    };

    // 30 / (650 * 0.04) = 1,153846154... ~1.15
    let result = payback_period(&metrics).unwrap_err();
    assert_eq!(result.code, 14);
    assert_eq!(result.name, String::from("InvalidARPUOrMArgin"));
}

#[test]
pub fn payback_period_correct() {
    let metrics = PaybackData {
        cac: dec!(30),
        arpu: dec!(650),
        margin: dec!(0.04),
    };

    // 30 / (650 * 0.04) = 1,153846154... ~1.15
    let result = payback_period(&metrics).unwrap().round_dp(2);
    assert_eq!(result, dec!(1.15));
}

// Break-even
#[test]
pub fn break_even_correct() {
    //Break-even (клиенты) = Fixed Costs / (ARPU × Margin − CAC × Churn Rate)
    let metrics = BreakEvenData {
        fixed_costs: dec!(100000),
        arpu: dec!(650),
        margin: dec!(0.04),
        cac: dec!(50),
        churn_rate: dec!(0.13),
    };

    //100000 / (650 * 0.04 - 50 * 0.13) = 5128,205128205... ~5128.21... ~5129
    let result = break_even(&metrics).unwrap();
    assert_eq!(result, dec!(5129));
}

#[test]
pub fn break_even_correct_den_0() {
    //Break-even (клиенты) = Fixed Costs / (ARPU × Margin − CAC × Churn Rate)
    let metrics = BreakEvenData {
        fixed_costs: dec!(100000),
        arpu: dec!(650),
        margin: dec!(0),
        cac: dec!(50),
        churn_rate: dec!(0.13),
    };

    let result = break_even(&metrics).unwrap_err();
    assert_eq!(result.code, 15);
    assert_eq!(result.name, String::from("InvalidBreakEvenDenominator"));
}

// Obfuscate
#[test]
pub fn test_obfuscate() {
    let data = b"Hello, UnitVault!";
    let key = b"secret_key_123";

    let res = obfuscate(data, key);
    let deobfuscate = deobfuscate(&res[..], key);

    assert_eq!(data.to_vec(), deobfuscate);
}

#[test]
pub fn test_save_and_load_file() {
    let model: UnitModel = UnitModel {
        id: String::from(Uuid::now_v7()),
        name: "TestFile".to_string(),
        business_type: BusinessType::SaaS,
        created_at: 0,
        updated_at: 0,
        inputs: ModelInputs {
            cac: dec!(1235.78),
            arpu: dec!(2750),
            margin: dec!(0.04),
            churn_rate: dec!(0.2),
            time_horizon: 24,
            fixed_costs: None,
            expansion_revenue: None,
            discount_rate: None,
            aov: None,
            purchase_frequency: None,
            take_rate: None,
        },
    };

    save("test.uvault", &model, "password").unwrap();
    let loading: UnitModel = load("test.uvault", "password").unwrap();

    assert_eq!(model, loading);
}
