#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/usr/:id/:user", axum::routing::get(handler_param));

    // 以下は実装例
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 必要な関数の宣言
async fn handler_top() -> String {
    "Hello, World!".to_string()
}

async fn handler_param(/* パラメータ宣言 */) -> String {
    "実装例".to_string()
}
