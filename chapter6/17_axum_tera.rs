#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(handle_index));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
