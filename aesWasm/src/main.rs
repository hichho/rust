use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::encode;
use std::result::Result;

//key:test1
//iv:test2
//password:test3
//expectedResult:test4
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum EncryptionError {
    /// The key or IV is the wrong length.
    BadKeyOrIv,
}

pub fn aes_128_cbc_encrypt(
    ptext: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, EncryptionError> {
    Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)
        .map_err(|_| EncryptionError::BadKeyOrIv)
        .map(|mode| mode.encrypt_vec(ptext))
}

fn main() {
    let key = String::from("test1");
    let key_vec = key.as_bytes();

    let iv = String::from("test2");
    let iv_vec = iv.as_bytes();

    let password = String::from("test3");
    let password_vec = password.as_bytes();

    let result = aes_128_cbc_encrypt(&password_vec, &key_vec, &iv_vec).expect("valid key and IV");
    assert_eq!(
        encode(result.clone()),
        "test4"
    );
    println!("Plaintext: {:?}", encode(&result));
    println!("Key: {:?}", encode(&key));
    println!("IV: {:?}", encode(&iv));
}
