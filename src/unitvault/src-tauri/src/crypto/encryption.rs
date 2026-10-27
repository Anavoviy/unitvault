use crate::error_handler::Error;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, AesGcm, Key, KeyInit, Nonce};
use argon2::Algorithm::Argon2id;
use argon2::{Argon2, Params, Version};
use rand::Rng;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use crate::crypto::obfuscation::obfuscate;
use crate::error_handler::store_errors::StoreError;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedData {
    pub magic: [u8; 4],
    pub version: u8,
    pub salt: [u8; 16],
    pub iv: [u8; 12],
    pub encrypted_data: Vec<u8>,
    pub hmac: [u8; 32],
}

impl EncryptedData {
    pub fn default() -> EncryptedData {
        EncryptedData {
            magic: *b"UVAU",
            version: 0,
            salt: [0u8; 16],
            iv: [0u8; 12],
            encrypted_data: vec![],
            hmac: [0u8; 32],
        }
    }

    pub fn new(version: u8) -> Result<EncryptedData, Error> {
        let mut rng = rand::rng();

        let mut salt = [0u8; 16];
        let mut iv = [0u8; 12];

        rng.fill_bytes(&mut salt);
        rng.fill_bytes(&mut iv);

        let mut data = EncryptedData {
            magic: *b"UVAU",
            version,
            salt,
            iv,
            encrypted_data: vec![],
            hmac: [0u8; 32],
        };

        Ok(data)
    }

    pub fn encrypt(&mut self, password: &[u8]) -> Result<Vec<u8>, Error> {
        let mut key = [0u8; 32];

        let params = Params::new(15, 8, 1, None).unwrap();
        let argon2 = Argon2::new(Argon2id, Version::V0x13, params);
        argon2
            .hash_password_into(password, &self.salt, &mut key)
            .unwrap();

        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        let nonce = Nonce::from_slice(&self.iv);

        //TODO: Добавить обфускацию
        let mut ciphertext = cipher
            .encrypt(nonce, self.encrypted_data.as_slice())
            .unwrap();

        let mut file = Vec::new();
        file.append(&mut self.magic.to_vec());
        file.push(self.version);
        file.append(&mut self.salt.to_vec());
        file.append(&mut self.iv.to_vec());
        file.append(&mut ciphertext);

        self.hmac = hmac_sha256::HMAC::mac(&file.as_slice(), &key);

        file.append(&mut self.hmac.to_vec());

        Ok(file)
    }

    pub fn decrypt(&mut self, data: Vec<u8>, password: &[u8]) -> Result<u8, Error> {
        let len = data.len();
        if len < 66 || data[0..4] != *b"UVAU" {
            return Err(Error::from(StoreError::FileCrashed));
        }

        if data[4] > self.version {
            return Err(Error::from(StoreError::VersionInvalid));
        }

        let mut key = [0u8; 32];

        self.salt.copy_from_slice(&data[5..21]);
        self.iv.copy_from_slice(&data[21..33]);
        self.hmac.copy_from_slice(&data[(len - 32)..len]);

        let params = Params::new(15, 8, 1, None).unwrap();
        let argon2 = Argon2::new(Argon2id, Version::V0x13, params);
        argon2
            .hash_password_into(password, &self.salt, &mut key)
            .unwrap();

        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        let nonce = Nonce::from_slice(&self.iv);

        let hmac = hmac_sha256::HMAC::mac(&data[..(len - 32)], &key);
        if hmac != self.hmac {
            return Err(Error::from(StoreError::FileCrashed));
        }

        self.encrypted_data = cipher
            .decrypt(nonce, &data[33..(len - 32)])
            .expect("Хуй знает, как так получилось");

        Ok(0)
    }
}
