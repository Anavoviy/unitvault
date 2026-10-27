import {UnitModel} from "./unitmodel.ts";
import {CalculatedMetrics} from "./calculate_metrics.ts";

export type CreateModelData = {
    name: String,
    model_type: number,
    cac: number,
    arpu: number,
    margin: number,
    churn_rate: number
}

export type ResponceCreateModel = {
    model: UnitModel,
    calc: CalculatedMetrics
}