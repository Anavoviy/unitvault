export enum MetricStatus {
    Critical = 1,
    Warning = 2,
    Good = 3,
    Excellent = 4,
    Info = 0
}

export type CalculateMetrics = {
    ltv: number,
    ltv_cac_ratio: number,
    payback_period: number,
    break_even_point: number,
    nrr: number | null,

    //ltv_cac_status: MetricStatus,
    //payback_status: MetricStatus
}