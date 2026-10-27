mod models;

use crate::error_handler::Error;
use tauri::{App, AppHandle, Runtime, State};
use std::sync::Mutex;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tauri_plugin_dialog::DialogExt;
use crate::AppState;
use crate::commands::models::{RequestCreateModel, ResponseCalculatedMetrics, ResponseUnitModel};
use crate::error_handler::model_errors::ModelError;
use crate::local_store::read_from_file;
use crate::models::{BusinessType, EcomInputs, MarketplaceInputs, SaaSInputs, UnitModel};

#[tauri::command]
pub async fn get_open_filepath(app: AppHandle) -> Result<String, String> {
    // Выбор файла в диалоговом окне
    //
    //TODO: Нужно проверить
    let filepath = match app.dialog()
        .file()
        .add_filter("Файл unitvault (*.uvault)", &["uvault"])
        .blocking_pick_file(){
            Some(f) => f.to_string(),
            None => return Err("Файл не выбран".to_string())
        };

    Ok(filepath)
}

#[tauri::command]
pub fn load_file(file_path: String, password: String, app_state: State<Mutex<AppState>>) -> Result<String, String> {
    // Загрузка из файла с использованием пароля
    //TODO: Проверить
    let model = match read_from_file(file_path, password) {
        Ok(m) => m,
        Err(e) => return Err(e.message)
    };

    let mut state = app_state.lock().unwrap();
    state.current_model = model;

    Ok("success".to_string())
}

#[tauri::command]
pub fn get_current_model(app_state: State<Mutex<AppState>>) -> Result<ResponseUnitModel, String> {
    let state = match app_state.lock() {
        Ok(m) => m,
        Err(e) => return Err("Модель не выбрана".to_string())
    };

    let model: UnitModel = state.current_model.clone();

    Ok(model.to_dto())
}

#[tauri::command]
pub fn create_new_model(dto: RequestCreateModel, app_state: State<Mutex<AppState>>)
    -> Result<ResponseUnitModel, String> {
    // Проверка полей и создание модели
    let mut model = match dto.business_type{
        BusinessType::SaaS => UnitModel::create_saas(dto.name, SaaSInputs {
            cac: dto.inputs.cac,
            margin: dto.inputs.margin,
            time_horizon: match dto.inputs.time_horizon {
                0 => 24,
                t => t
            },
            arpu: match dto.inputs.arpu {
                Some(a) => a,
                None => return Err(Error::from(ModelError::ArpuNotFound).message)
            },
            churn_rate: match dto.inputs.churn_rate {
                Some(a) => a,
                None => return Err(Error::from(ModelError::ChurnRateNotFound).message)
            },
            fixed_cost: dto.inputs.fixed_cost,
            expansion_revenue: dto.inputs.expansion_revenue,
        }),
        BusinessType::Ecommerce => UnitModel::create_ecom(dto.name, EcomInputs {
            cac: dto.inputs.cac,
            margin: dto.inputs.margin,
            time_horizon: match dto.inputs.time_horizon {
                0 => 24,
                t => t
            },
            arpu: match dto.inputs.arpu {
                Some(a) => a,
                None => return Err(Error::from(ModelError::ArpuNotFound).message)
            },
            churn_rate: match dto.inputs.churn_rate {
                Some(a) => a,
                None => return Err(Error::from(ModelError::ChurnRateNotFound).message)
            },
            aov: match dto.inputs.aov {
                Some(a) => a,
                None => return Err(Error::from(ModelError::AovNotFound).message)
            },
            purchase_frequency: match dto.inputs.purchase_frequency {
                Some(a) => a,
                None => return Err(Error::from(ModelError::PurchaseFrequencyNotFound).message)
            },
            fixed_cost: dto.inputs.fixed_cost,
        }),
        BusinessType::Marketplace => UnitModel::create_marketplace(dto.name, MarketplaceInputs {
            cac: dto.inputs.cac,
            margin: dto.inputs.margin,
            time_horizon: match dto.inputs.time_horizon {
                0 => 24,
                t => t
            },
            aov: match dto.inputs.aov {
                Some(a) => a,
                None => return Err(Error::from(ModelError::AovNotFound).message)
            },
            purchase_frequency: match dto.inputs.purchase_frequency {
                Some(a) => a,
                None => return Err(Error::from(ModelError::PurchaseFrequencyNotFound).message)
            },
            take_rate: match dto.inputs.take_rate {
                Some(a) => a,
                None => return Err(Error::from(ModelError::TakeRateNotFound).message)
            },
            fixed_cost: dto.inputs.fixed_cost,
            expansion_revenue: dto.inputs.expansion_revenue,
        }),
        _ => return Err(Error::from(ModelError::BusinessTypeNotSet).message)
    };

    // Расчет метрик
    match model.calculate() {
        Err(e) => return Err(e.message),
        _ => {}
    };

    // Сохранение модели
    //TODO: Проверить!!!
    let mut state = app_state.lock().unwrap();
    state.current_model = model.clone();

    Ok(model.to_dto())
}

#[tauri::command]
pub fn update_model(property: String, value: String, app_state: State<Mutex<AppState>>)
    -> Result<ResponseCalculatedMetrics, String> {
    //TODO: Проверить!!!
    let mut state = app_state.lock().unwrap();

    // Обновление поля модели
    match property.as_str() {
        "name" => state.current_model.name = value,

        // Обязательные метрикиЫ
        "cac" => state.current_model.inputs.cac = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => d,
            Err(e) => return Err(Error::from(ModelError::CacParseInvalid).message)
        },
        "margin" => state.current_model.inputs.margin = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => d,
            Err(e) => return Err(Error::from(ModelError::MarginParseInvalid).message)
        },
        "time_horizon" => state.current_model.inputs.time_horizon = match value.parse() {
            Ok(d) => d,
            Err(e) => return Err(Error::from(ModelError::TimeHorizonParseInvalid).message)
        },

        // Опциональные метрики
        "arpu" => state.current_model.inputs.arpu = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::ArpuParseInvalid).message)
        },
        "churn_rate" => state.current_model.inputs.churn_rate = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::ChurnRateParseInvalid).message)
        },
        "aov" => state.current_model.inputs.aov = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::AovParseInvalid).message)
        },
        "purchase_frequency" => state.current_model.inputs.purchase_frequency = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::PurchaseFrequencyParseInvalid).message)
        },

        "fixed_cost" => state.current_model.inputs.fixed_cost = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::FixedCostParseInvalid).message)
        },
        "expansion_revenue" => state.current_model.inputs.expansion_revenue = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::ExpansionRevenueParseInvalid).message)
        },
        "take_rate" => state.current_model.inputs.take_rate = match Decimal::from_str_exact(value.as_str()) {
            Ok(d) => Some(d),
            Err(e) => return Err(Error::from(ModelError::TakeRateParseInvalid).message)
        },
        _ => return Err(String::from("Указанный параметр модели отсутствует, пупупс"))
    };

    // Пересчет метрик
    Ok(
        ResponseCalculatedMetrics {
            id: state.current_model.id.clone(),
            calculated: match state.current_model.calculate() {
                Err(e) => return Err(e.message),
                Ok(v) => v.clone()
            },
        })
}

