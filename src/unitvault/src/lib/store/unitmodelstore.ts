const UnitModelStore = ( () => {
    let model: UnitModel | null = null;

    function setInstance(data: UnitModel) {
        model = data;
    }

    function createModel(): UnitModel {
        return {
            id: "id",
            name: "empty_name",
            business_type: BussinessType.Custom,
            inputs: {
                cac: 0,
                arpu: 0,
                margin: 0,
                churn_rate: 0,
                time_horizons: 0,
                fixed_costs: null,
                expansion_revenue: null,
                discount_rate: null,
                aov: null,
                purchase_frequency: null,
                take_rate: null
            }
        };
    }

    return {
        getModel: () => {
            if(!model) {
                model = createModel();
            }
            return model;
        }
    }
})();