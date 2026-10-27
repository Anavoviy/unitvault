use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error_handler::{Error, model_errors::ModelError};
use crate::models::{BusinessType, CalculateMetrics, UnitModel};

impl UnitModel {
    pub fn ltv(&self) -> Result<Decimal, Error> {
        let mut ltv: Decimal = dec!(0);

        if self.business_type == BusinessType::SaaS {
            let arpu = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::ArpuNotFound))
            };
            let churn_rate = match self.inputs.churn_rate {
                Some(churn_rate) => churn_rate,
                None => return Err(Error::from(ModelError::ChurnRateNotFound))
            };
            ltv = (arpu * self.inputs.margin)
                / churn_rate;
        } else if self.business_type == BusinessType::Ecommerce {
            let aov = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::AovNotFound))
            };
            let purchase_frequency = match self.inputs.purchase_frequency {
                Some(purchase_frequency) => purchase_frequency,
                None => return Err(Error::from(ModelError::PurchaseFrequencyNotFound))
            };
            let customer_lifespan = match self.inputs.churn_rate {
                Some(churn_rate) => dec!(1) / churn_rate,
                None => return Err(Error::from(ModelError::ChurnRateNotFound))
            };
            ltv = aov *
                self.inputs.margin *
                purchase_frequency *
                (dec!(1) / customer_lifespan);
        } else if self.business_type == BusinessType::Marketplace {
            let aov = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::AovNotFound))
            };
            let purchase_frequency = match self.inputs.purchase_frequency {
                Some(purchase_frequency) => purchase_frequency,
                None => return Err(Error::from(ModelError::PurchaseFrequencyNotFound))
            };
            let take_rate = match self.inputs.take_rate {
                Some(take_rate) => take_rate,
                None => return Err(Error::from(ModelError::TakeRateNotFound))
            };
            let customer_lifespan = match self.inputs.churn_rate {
                Some(churn_rate) => dec!(1) / churn_rate,
                None => return Err(Error::from(ModelError::ChurnRateNotFound))
            };
            ltv = aov *
                self.inputs.take_rate.unwrap() *
                purchase_frequency *
                self.inputs.margin *
                customer_lifespan;
        } else {
            //TODO: Сделать отработку для варианта Custom
        }

        Ok(ltv)
    }
    pub fn ltv_cac(&self) -> Result<Decimal, Error> {
        let ltv = match self.ltv() {
            Ok(ltv) => ltv,
            Err(e) => return Err(e)
        };

        let ltv_cac: Decimal = ltv / self.inputs.cac;

        Ok(ltv_cac)
    }

    pub fn payback_period(&self) -> Result<Decimal, Error> {
        let mut payback: Decimal = dec!(0);

        if self.business_type == BusinessType::SaaS{
            let arpu = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::ArpuNotFound))
            };
            payback = self.inputs.cac / (arpu * self.inputs.margin);
        } else if self.business_type == BusinessType::Ecommerce || self.business_type == BusinessType::Marketplace {
            let aov = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::AovNotFound))
            };
            let purchase_frequency = match self.inputs.purchase_frequency {
                Some(purchase_frequency) => purchase_frequency,
                None => return Err(Error::from(ModelError::PurchaseFrequencyNotFound))
            };

            // Здесь учитывается, будто purchase_frequency берется за год
            //TODO: Проработать момент с purshace_frequency, чтобы он настраивался и выбирался период, за который он берется
            payback = self.inputs.cac / ((aov * purchase_frequency / dec!(12)) * self.inputs.margin);
        } else {
            //TODO: Сделать отработку для варианта Custom
        }

        Ok(payback)
    }

    pub fn break_even_p(&self) -> Result<Decimal, Error> {
        let mut break_even: Decimal = dec!(0);

        if self.business_type == BusinessType::SaaS{
            let arpu = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::ArpuNotFound))
            };
            let fixed_cost = match self.inputs.fixed_cost{
                Some(fixed_cost) => fixed_cost,
                None => return Err(Error::from(ModelError::FixedCostNotFound))
            };

            break_even = fixed_cost / (arpu * self.inputs.margin);
        } else if self.business_type == BusinessType::Ecommerce {
            //TODO: Сделать расчет относительно AOV
             let arpu = match self.inputs.arpu {
                Some(arpu) => arpu,
                None => return Err(Error::from(ModelError::ArpuNotFound))
            };
            let fixed_cost = match self.inputs.fixed_cost{
                Some(fixed_cost) => fixed_cost,
                None => return Err(Error::from(ModelError::FixedCostNotFound))
            };

            break_even = fixed_cost / (arpu * self.inputs.margin);
        } else if self.business_type == BusinessType::Marketplace {
            let aov = match self.inputs.arpu {
                Some(aov) => aov,
                None => return Err(Error::from(ModelError::AovNotFound))
            };
            let fixed_cost = match self.inputs.fixed_cost{
                Some(fixed_cost) => fixed_cost,
                None => return Err(Error::from(ModelError::FixedCostNotFound))
            };
            let take_rate = match self.inputs.take_rate {
                Some(take_rate) => take_rate,
                None => return Err(Error::from(ModelError::TakeRateNotFound))
            };

            break_even = fixed_cost / ((aov * self.inputs.margin) - (aov * take_rate));
        }

        Ok(break_even)
    }

    pub fn calculate(&mut self) -> Result<&CalculateMetrics, Error> {
        let ltv = match self.ltv() {
            Ok(ltv) => ltv,
            Err(err) => return Err(err)
        };

        let ltv_cac = match self.ltv_cac() {
            Ok(ltv_cac) => ltv_cac,
            Err(err) => return Err(err)
        };

        let payback = match self.payback_period(){
            Ok(payback) => payback,
            Err(err) => return Err(err)
        };

        let break_even = match self.break_even_p(){
            Ok(break_even) => break_even,
            Err(err) => return Err(err)
        };

        let calc = CalculateMetrics {
            ltv,
            ltv_cac,
            payback_period: payback,
            break_even_point: break_even,
            nrr: None,
        };

        self.calculated = calc;

        Ok(&self.calculated)
    }
}