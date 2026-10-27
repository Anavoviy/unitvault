use std::fs::File;
use std::io::{Read, Write};
use serde::de::Unexpected::Unit;
use crate::crypto::encryption::EncryptedData;
use crate::models::UnitModel;
use crate::error_handler::{Error, store_errors::StoreError};


pub fn read_from_file(path: String, password: String) -> Result<UnitModel, Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(Error::from(StoreError::FileUnExist))
    };

    let mut data: Vec<u8> = Vec::new();

    match &file.read_to_end(&mut data) {
        Err(e) => return Err(Error::from(StoreError::FileCrashed)),
        _ => {}
    };

    let mut encypted: EncryptedData = EncryptedData::new(1).unwrap();

    match &encypted.decrypt(data, password.as_bytes()) {
        Err(err) => return Err(Error::from(StoreError::FileCrashed)),
        _ => {}
    };

    let model: UnitModel = match serde_json::from_slice(encypted.encrypted_data.as_slice()) {
        Ok(m) => m,
        Err(e) => return Err(Error::from(StoreError::FileCrashed))
    };

    Ok(model)
}

pub fn save_to_file(model: &UnitModel, path: String, password: String) -> Result<(), Error> {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(Error::from(StoreError::FileUnExist))
    };

    let data = match serde_json::to_vec(model){
        Ok(d) => match serde_json::to_vec(&d) {
            Ok(b) => b,
            Err(e) => return Err(Error::from(StoreError::ModelCrashed))
        },
        Err(e) => return Err(Error::from(StoreError::ModelCrashed))
    };

    let mut encypted: EncryptedData = match EncryptedData::new(1){
        Ok(en) => en,
        Err(e) => return Err(Error::from(StoreError::ModelCrashed))
    };

    encypted.encrypted_data = data;
    let filedata = match encypted.encrypt(password.as_bytes()) {
        Err(e) => return Err(Error::from(StoreError::EncyptCrashed)),
        Ok(f) => f
    };

    match file.write_all(filedata.as_slice()) {
        Err(e) => return Err(Error::from(StoreError::EncyptCrashed)),
        _ => {}
    };
    
    Ok(())
}