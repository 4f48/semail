use crate::common::opaque::Default;
use opaque_ke::keypair::PrivateKey;
use opaque_ke::{Ristretto255, ServerSetup};
use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::Arc;

pub struct AppState {
    server_setup: ServerSetup<Default, PrivateKey<Ristretto255>>
}

pub fn main() -> Arc<AppState> {
    let mut rng = OsRng;
    let server_setup = ServerSetup::<Default>::new(&mut rng);
    Arc::new(AppState { server_setup })
}