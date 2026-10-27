use serde::{Deserialize, Serialize};
use crate::models::{BusinessType, CalculateMetrics, InputMetrics, UnitModel};

#[derive(Serialize, Debug)]
pub struct ResponseUnitModel {
    pub id: String,
    pub name: String,
    pub business_type: BusinessType,

    pub inputs: InputMetrics,
    pub calculated: CalculateMetrics,
}

impl UnitModel {
    pub fn to_dto(&self) -> ResponseUnitModel {
        ResponseUnitModel {
            id: self.id.clone(),
            name: self.name.clone(),
            business_type: self.business_type.clone(),
            inputs: self.inputs.clone(),
            calculated: self.calculated.clone(),
        }
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct RequestCreateModel {
    pub name: String,
    pub business_type: BusinessType,

    pub inputs: InputMetrics,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseCalculatedMetrics {
    pub id: String,
    pub calculated: CalculateMetrics
}