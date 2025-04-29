async fn handler_param(axum::extract::Path(user_id): axum::extract::Path<String>) -> String {
    format!("User ID: {}", user_id)
}
