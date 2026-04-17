use std::io::Error;

use openssl::error::ErrorStack;

use crate::util::key::{SK, generate_pk_and_sk};

const NODE_KEY_BITS: u32 = 512;

const NODE_KEY_PATH: &str = "node/key.der";

pub fn load_key() -> Result<SK, ()> {
    if std::fs::metadata(NODE_KEY_PATH).is_ok() {
        read_key().map_err(|_| ())
    } else {
        match generate_pk_and_sk(NODE_KEY_BITS) {
            Ok((_, sk)) => {
                save_key(&sk).map_err(|_| ())?;
                Ok(sk)
            }
            Err(_) => Err(()),
        }
    }
}

pub fn generate_key() -> Result<SK, ErrorStack> {
    generate_pk_and_sk(NODE_KEY_BITS).map(|(_, sk)| sk)
}

pub fn read_key() -> Result<SK, Error> {
    std::fs::read_to_string(NODE_KEY_PATH).map(|der| SK { der: der })
}

pub fn save_key(sk: &SK) -> Result<(), Error> {
    std::fs::write(NODE_KEY_PATH, &sk.der)
}
