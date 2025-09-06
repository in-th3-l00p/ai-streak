use crate::service::auth_service::AuthService;
use axum::http::HeaderMap;
use common::domain::user::User;

pub fn get_authenticated_user(
    headers: HeaderMap,
    auth_service: &AuthService
) -> Option<User> {
    headers
        .get("Authorization")
        .and_then(|h| h
            .to_str()
            .ok()
            .and_then(|jwt| {
                jwt
                    .strip_prefix("Bearer ")
                    .map(|token| token.to_owned())
            }))
        .and_then(|jwt| auth_service.get_user(jwt).ok())
}