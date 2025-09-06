use std::collections::BTreeMap;
use anyhow::anyhow;
use common::domain::user::User;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;

pub const USER_KEY: &str = "user";

pub struct AuthService {
    key: Hmac<Sha256>,
}

impl AuthService {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            key: Hmac::new_from_slice(secret)
                .expect("error creating the hmac for jwts, try to improve the secret's strength"),
        }
    }

    pub fn sign(self: &Self, user: User) -> anyhow::Result<String> {
        Ok(BTreeMap::from([(USER_KEY.to_string(), serde_json::to_string(&user)?)])
            .sign_with_key(&self.key)?)
    }

    pub fn get_user(self: &Self, jwt: String) -> anyhow::Result<User> {
        let token: Token<jwt::Header, BTreeMap<String, String>, _> = jwt.verify_with_key(&self.key)?;
        Ok(serde_json::from_str::<User>(
            &token
                .claims()
                .get(USER_KEY)
                .ok_or(anyhow!("invalid jwt"))?
        )?)
    }
}
