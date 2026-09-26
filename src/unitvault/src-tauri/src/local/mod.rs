use crate::crypto::encryption::EncryptedData;
use crate::types::{BusinessType, Error, ModelInputs, UnitModel};
use rmp_serde::Serializer;
use rust_decimal_macros::dec;
use serde::Serialize;
use std::fs::File;
use std::io::{Read, Write};
use uuid::Uuid;
use tauri_plugin_dialog::DialogExt;

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