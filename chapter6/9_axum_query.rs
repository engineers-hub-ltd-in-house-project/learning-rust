async fn handler_query(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> String {
    format!("id:{}, name:{}.", params["id"], params["name"])
}
