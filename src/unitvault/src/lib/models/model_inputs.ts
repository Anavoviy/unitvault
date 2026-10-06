type ModelInputs = {
    cac: number,
    arpu: number,
    margin: number,
    churn_rate: number,

    time_horizons: number,
    fixed_costs: number | null,
    expansion_revenue: number | null,
    discount_rate: number | null,
    aov: number | null,
    purchase_frequency: number | null,
    take_rate: number | null,
}