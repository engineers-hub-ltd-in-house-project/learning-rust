/*
 * Axumハンドラー関数の実装
 *
 * この例では、Axumフレームワークを使用してHTTPリクエストを処理するための
 * ハンドラー関数の実装方法を示しています。
 *
 * ハンドラー関数の特徴：
 * 1. HTTPリクエストを受け取り、レスポンスを返す関数
 * 2. 非同期関数として実装される（async fn）
 * 3. 様々な型のレスポンスを返すことができる
 * 4. 複数のハンドラーを異なるルートに割り当てることができる
 *
 * この例では、ルートパス（"/"）へのGETリクエストとinfo、aboutパスへのリクエストに
 * 対応する3つのハンドラー関数を定義しています。
 */

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 複数のルートとハンドラーを持つルーターを設定
    let app = Router::new()
        .route("/", get(index_handler)) // ルートパスへのGETリクエスト
        .route("/info", get(info_handler)) // /infoパスへのGETリクエスト
        .route("/about", get(about_handler)); // /aboutパスへのGETリクエスト

    // サーバーを起動
    let addr = "127.0.0.1:3000";
    println!("サーバーを起動しています: {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ルートパスのハンドラー関数
async fn index_handler() -> &'static str {
    "ホームページへようこそ"
}

// /infoパスのハンドラー関数
async fn info_handler() -> &'static str {
    "ここは情報ページです"
}

// /aboutパスのハンドラー関数
async fn about_handler() -> &'static str {
    "このサイトについて"
}
