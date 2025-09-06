use crate::app::AppState;

pub async fn seed() {
    let app = AppState::new().await;
    let user = app.user_service.create(
        "intheloop".to_string(),
        "admin@tiscacatalin.com".to_string(),
        "password".to_string(),
    ).await;
    match user {
        Ok(user) =>
            tracing::info!(
                "created user {} with email: {} and password: password",
                user.username, user.email
            ),
        Err(e) =>
            tracing::error!("failed to create user: {}", e)
    }
}