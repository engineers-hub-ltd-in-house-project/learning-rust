let app = axum::Router::new()
  .route("/", axum::routing::get(handle_index))
  .route("/post", axum::routing::post(handle_post)); 