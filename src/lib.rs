use std::{io::Read, io::Write};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(passphrase: &str, file: Vec<u8>) -> Vec<u8> {
    let encryptor =
        age::Encryptor::with_user_passphrase(age::secrecy::Secret::new(passphrase.to_owned()));

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
    writer.write(&file).unwrap();
    writer.finish().unwrap();

    return encrypted;
}

#[wasm_bindgen]
pub fn decrypt(passphrase: &str, file: Vec<u8>) -> Vec<u8> {
    let decrypted = {
        let decryptor = match age::Decryptor::new(&file[..]).unwrap() {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor
            .decrypt(&age::secrecy::Secret::new(passphrase.to_owned()), None)
            .unwrap();
        reader.read_to_end(&mut decrypted).unwrap();

        decrypted
    };
    
    return decrypted;
}
