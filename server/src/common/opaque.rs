use argon2::Argon2;
use opaque_ke::{CipherSuite, Ristretto255};

pub struct Default;
impl CipherSuite for Default {
    type OprfCs = Ristretto255;
    type KeGroup = Ristretto255;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
    type Ksf = Argon2<'static>;
}
