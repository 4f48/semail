use argon2::password_hash::rand_core::OsRng;
use argon2::Argon2;
use base64::prelude::*;
use base64::Engine;
use opaque_ke::{CipherSuite, Ristretto255, ServerSetup};
use serde::Serialize;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};

#[derive(Serialize)]
pub struct Default;
impl CipherSuite for Default {
    type OprfCs = Ristretto255;
    type KeGroup = Ristretto255;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
    type Ksf = Argon2<'static>;
}

pub async fn server_setup() -> ServerSetup<Default> {
    match File::create_new("server_setup.txt") {
        Ok(mut file) => {
            println!("ServerSetup not found, generating...");
            let mut rng = OsRng;
            let server_setup = ServerSetup::<Default>::new(&mut rng);
            let serialized = bincode::serialize(&server_setup).unwrap();
            let encoded = BASE64_STANDARD.encode(serialized);
            file.write_all(encoded.as_bytes())
                .expect("Failed to write server setup to file");
            println!("Wrote ServerSetup to server_setup.txt");
            server_setup
        }
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("Found server_setup.txt, reading ServerSetup...");
                let mut file = File::open("server_setup.txt").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let decoded = BASE64_STANDARD.decode(contents).unwrap();
                println!("Ok");
                bincode::deserialize(&decoded).unwrap()
            }
            _ => panic!("Failed to read server setup from file"),
        },
    }
}
