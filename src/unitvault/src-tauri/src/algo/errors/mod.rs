use crate::types::Error;

pub fn ltv_churnrate_lower_0() -> Error {
    Error {
        name: String::from("InvalidChurnRate"),
        code: 11,
        message: String::from("Процент ушедших клиентов за период равен или меньше 0"),
    }
}

pub fn ltv_ecommerce_parts_lower_0() -> Error {
    Error {
        name: String::from("InvalidParametersLtvECommerce"),
        code: 12,
        message: String::from("Churn Rate, AOV, Purchase Frequency или Margin равны или меньше 0"),
    }
}

pub fn ltvcac_cac_lower_0() -> Error {
    Error {
        name: String::from("InvalidCac"),
        code: 13,
        message: String::from("Значение CAC равно или меньше 0"),
    }
}

pub fn payback_arpu_or_margin_lower_0() -> Error {
    Error {
        name: String::from("InvalidARPUOrMArgin"),
        code: 14,
        message: String::from("Значение ARPU или Margin равно или меньше 0"),
    }
}

pub fn break_even_denominator_0() -> Error {
    Error {
        name: String::from("InvalidBreakEvenDenominator"),
        code: 15,
        message: String::from(
            "Значение делителя в расчете точки безубыточности равно или меньше 0",
        ),
    }
}
