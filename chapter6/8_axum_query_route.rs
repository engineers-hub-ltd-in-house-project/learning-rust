#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/qry", axum::routing::get(handler_query));

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

async fn handler_query(/* クエリパラメータ */) -> String {
    "クエリパラメータの処理例".to_string()
}
