use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct AuthService {
    hmac: Hmac<Sha256>,
}

impl AuthService {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            hmac: Hmac::new_from_slice(secret)
                .expect("error creating the hmac for jwts, try to improve the secret's strength"),
        }
    }
}
