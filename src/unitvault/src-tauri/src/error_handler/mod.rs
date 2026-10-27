pub mod model_errors;
pub mod store_errors;
pub mod algo_errors;

use model_errors::ModelError;
use store_errors::StoreError;
use algo_errors::AlgoError;

#[derive(Default, Debug)]
pub struct Error {
    name: String,
    code: i32,
    pub message: String
    // function: current_fn - сохранять название функции, в которой появилась ошибка
}

impl From<ModelError> for Error {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::ArpuNotFound => Error {
                name: "ArpuNorFound".to_string(),
                code: 301,
                message: "Основной параметр ARPU, необходимый для расчетов, отсутствует в модели".to_string(),
            },
            ModelError::ChurnRateNotFound => Error {
                name: "ChurnRateNorFound".to_string(),
                code: 302,
                message: "Основной параметр Churn Rate, необходимый для расчетов, отсутствует в модели".to_string(),
            },
            ModelError::AovNotFound => Error {
                name: "AovNorFound".to_string(),
                code: 303,
                message: "Основной параметр AOV, необходимый для расчетов, отсутствует в модели".to_string(),
            },
            ModelError::PurchaseFrequencyNotFound => Error {
                name: "PurchaseFrequencyNotFound".to_string(),
                code: 304,
                message: "Основной параметр Purchase Frequency, необходимый для расчетов, отсутствует в модели".to_string(),
            },
            ModelError::TakeRateNotFound => Error {
                name: "TakeRateNotFound".to_string(),
                code: 305,
                message: "Основной параметр Take Rate, необходимый для расчетов, отсутствует в модели".to_string(),
            },
            ModelError::FixedCostNotFound => Error {
                name: "FixedCostNotFound".to_string(),
                code: 306,
                message: "Основной параметр Fixed Cost, необходимый для расчетов, отсутствует в модели".to_string(),
            },
            ModelError::BusinessTypeNotSet => Error {
                name: "BusinessTypeNotSet".to_string(),
                code: 307,
                message: "Не удалось определить тип модели".to_string(),
            },
            ModelError::CacParseInvalid => Error {
                name: "CacParseInvalid".to_string(),
                code: 308,
                message: "Значение параметра CAC не похоже на число, пупупу".to_string(),
            },
            ModelError::MarginParseInvalid => Error {
                name: "MarginParseInvalid".to_string(),
                code: 309,
                message: "Значение параметра Margin (маржинальности) не похоже на число, пупупу".to_string(),
            },
            ModelError::TimeHorizonParseInvalid => Error {
                name: "TimeHorizonParseInvalid".to_string(),
                code: 310,
                message: "Значение параметра горизонта планирования не похоже на число, пупупу".to_string(),
            },
            ModelError::ArpuParseInvalid => Error {
                name: "ArpuParseInvalid".to_string(),
                code: 311,
                message: "Значение параметра ARPU не похоже на число, пупупу".to_string(),
            },
            ModelError::ChurnRateParseInvalid => Error {
                name: "ChurnRateParseInvalid".to_string(),
                code: 312,
                message: "Значение параметра Churn rate (отток клиентов) не похоже на число, пупупу".to_string(),
            },
            ModelError::AovParseInvalid => Error {
                name: "AovParseInvalid".to_string(),
                code: 313,
                message: "Значение параметра AOV не похоже на число, пупупу".to_string(),
            },
            ModelError::PurchaseFrequencyParseInvalid => Error {
                name: "PurchaseFrequencyParseInvalid".to_string(),
                code: 314,
                //TODO: Написать перевод в сообщение
                message: "Значение параметра Purchase frequency () не похоже на число, пупупу".to_string(),
            },
            ModelError::FixedCostParseInvalid => Error {
                name: "FixedCostParseInvalid".to_string(),
                code: 315,
                message: "Значение параметра Fixed Cost (постоянные расходы) не похоже на число, пупупу".to_string(),
            },
            ModelError::ExpansionRevenueParseInvalid => Error {
                name: "ExpansionRevenueParseInvalid".to_string(),
                code: 316,
                //TODO: Написать перевод в сообщение
                message: "Значение параметра Expansion Revenue () не похоже на число, пупупу".to_string(),
            },
            ModelError::TakeRateParseInvalid => Error {
                name: "TakeRateParseInvalid".to_string(),
                code: 317,
                message: "Значение параметра Take rate (комиссия платформы) не похоже на число, пупупу".to_string(),
            },
            _ => Error {
                name: "UnitModelCrashed".to_string(),
                code: 300,
                message: "Модель повреждена".to_string(),
            }
        }
    }


}

impl From<StoreError> for Error {
    fn from(err: StoreError) -> Self {
        match err {
            StoreError::FileUnExist => Error {
                name: "FileUnExist".to_string(),
                code: 101,
                message: "Файл отсутствует по указанному пути".to_string(),
            },
            StoreError::FileCrashed => Error {
                name: "FileCrashed".to_string(),
                code: 102,
                message: "Файл поврежден".to_string(),
            },
            StoreError::ModelCrashed => Error {
                name: "ModelCrashed".to_string(),
                code: 103,
                message: "Модель создана с ошибками".to_string(),
            },
            StoreError::EncyptCrashed => Error {
                name: "EncyptCrashed".to_string(),
                code: 104,
                message: "Шифрование не удалось".to_string(),
            },
            StoreError::VersionInvalid => Error {
                name: "VersionInvalid".to_string(),
                code: 105,
                message: "Данная версия не поддерживается".to_string(),
            },
            _ => Error {
                name: "LocalStoreCrashed".to_string(),
                code: 100,
                message: "Файл поврежден".to_string(),
            }
        }
    }
}