async fn handler_param(
    axum::extract::Path((id, user)): axum::extract::Path<(usize, String)>,
) -> String {
    format!("User ID:{}. name:{}.", id, user)
}
