import {CalculateMetrics} from "./calculate_metrics.ts";

export enum BusinessType {
    SaaS = 0,
    Ecommerce = 1,
    Marketplace = 2,
    Custom = 3
}

export type UnitModel = {
    id: string,
    name: string,
    business_type: BusinessType,
    inputs: ModelInputs,
    calculated: CalculateMetrics,
    created_at: number,
    updated_at: number
}

export function dflt(){
    let inputs: ModelInputs = {
        aov: null,
        arpu: 500,
        cac: 5000,
        churn_rate: 4,
        discount_rate: null,
        expansion_revenue: 0,
        fixed_costs: null,
        margin: 15,
        purchase_frequency: null,
        take_rate: null,
        time_horizons: 24

    }
    let calculated: CalculateMetrics = {
        break_even_point: 0,
        ltv: 0,
        ltv_cac_ratio: 0,
        payback_period: 0,
        nrr: null
    }

    let model: UnitModel = {
        business_type: BusinessType.SaaS,
        calculated: calculated,
        created_at: 0,
        id: "",
        inputs: inputs,
        name: "",
        updated_at: 0
    }

    return model;
}

