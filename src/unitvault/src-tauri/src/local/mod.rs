use crate::crypto::encryption::EncryptedData;
use crate::types::{BusinessType, CalculatedMetrics, Error, ModelInputs, UnitModel};
use crate::algo::calculate_metrics;
use rmp_serde::Serializer;
use rust_decimal_macros::dec;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use rust_decimal::Decimal;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::AppState;

pub fn save(filename: &str, model: &UnitModel, password: &str) -> Result<u8, Error> {
    let mut encrypted_data = EncryptedData::default();

    model
        .serialize(&mut Serializer::new(&mut encrypted_data.encrypted_data))
        .unwrap();

    let bytes = encrypted_data.encrypt(password.as_bytes()).unwrap();

    let mut file = File::create(filename).unwrap();
    file.write_all(bytes.as_slice()).unwrap();

    Ok(0)
}

pub fn load(filename: &str, password: &str) -> Result<UnitModel, Error> {
    let mut input_file = File::open(filename).unwrap();
    let mut buffer = Vec::new(); // буфер для считывания данных

    input_file
        .read_to_end(&mut buffer)
        .expect("Что-то пошло не так!");

    let mut decrypted_data = EncryptedData::default();
    decrypted_data.decrypt(buffer, password.as_bytes()).unwrap();

    let model: UnitModel = rmp_serde::from_slice(decrypted_data.encrypted_data.as_slice()).unwrap();

    Ok(model)
}

#[tauri::command]
pub async fn enter_filepath(app: tauri::AppHandle) -> Result<String, String> {
    let file_path = app.dialog().file()
        .add_filter("Файлы юнит-экономики (*.uvault)", &["uvault"])
        .blocking_pick_file()
        .unwrap();

    Ok(file_path.to_string())
}

struct CreateModelData {
    name: String,
    model_type: i32,
    cac: Decimal,
    arpu: Decimal,
    margin: Decimal,
    churn_rate: Decimal
}

#[tauri::command]
pub fn create_new_model(
    state: State<Mutex<AppState>>,
    create_data: CreateModelData
) -> Result<CalculatedMetrics, String>{
    // Создать модель
    let model = UnitModel::new(
        create_data.name,
        match create_data.model_type {
            0 => BusinessType::SaaS,
            1 => BusinessType::Ecommerce,
            2 => BusinessType::Marketplace,
            3 => BusinessType::Custom,
            _ => return Err(String::from("Не существует такого типа модели!"))
        },
        ModelInputs {
            cac: create_data.cac,
            arpu: create_data.arpu,
            margin: create_data.margin,
            churn_rate: create_data.churn_rate,

            time_horizon: 24,
            fixed_costs: None,
            expansion_revenue: None,
            discount_rate: None,
            aov: None,
            purchase_frequency: None,
            take_rate: None
        }
    ).unwrap();

    // Просчитать calculated_metrics
    let calculated = calculate_metrics(&model).unwrap();

    // Запихать её в AppStore
    {
        let mut app_state = state.inner().lock().unwrap();
        app_state.model = model;
    }

    Ok(calculated)
}

#[tauri::command]
pub fn calculate(state: State<Mutex<AppState>>, property: String, value: Decimal) -> Result<CalculatedMetrics, Error> {

    

    Ok(CalculatedMetrics {
        ltv: dec!(100),
        ltv_cac_ratio: dec!(100),
        payback_period: dec!(100),
        break_even_point: None,
        net_revenue_retention: None,
        ltv_cac_status: Default::default(),
        payback_status: Default::default(),
    })
}