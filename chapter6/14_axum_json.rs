/*
 * AxumでのJSONレスポンス実装
 *
 * この例では、Axumフレームワークを使用してHTTPリクエストに対して
 * JSONデータを返す方法を示しています。
 * 
 * JSONレスポンスの特徴：
 * 1. WebAPIで一般的に使用されるデータ形式
 * 2. axum::extract::JsonとSerdeを組み合わせて実装
 * 3. 構造体を自動的にJSONに変換できる
 * 4. クライアントサイドのJavaScriptなどで簡単に扱える
 * 
 * この例では、構造体をJSON形式に変換してレスポンスとして
 * 返すハンドラー関数を実装しています。
 */

use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;

// JSONで返すデータの構造体
#[derive(Serialize)]
struct MyData {
    id: u32,
    name: String,
    active: bool,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() {
    // ルーターの設定
    let app = Router::new()
        .route("/data", get(handler_data))           // 基本的なJSONデータを返すルート
        .route("/data/:id", get(handler_data_id));    // パラメータ付きのJSONデータを返すルート

    // サーバーの起動
    let addr = "127.0.0.1:3000";
    println!("JSONサーバーを起動しています: {}", addr);
    
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 基本的なJSONデータを返すハンドラー
async fn handler_data() -> Json<MyData> {
    // サンプルデータを作成
    let data = MyData {
        id: 1,
        name: String::from("サンプルデータ"),
        active: true,
        tags: vec![String::from("サンプル"), String::from("テスト")],
    };
    
    // Json型でラップして返す
    Json(data)
}

// IDをパスパラメータから取得してJSONデータを返すハンドラー
async fn handler_data_id(Path(id): Path<u32>) -> Json<MyData> {
    // パスパラメータのIDを使用してデータを作成
    let data = MyData {
        id,
        name: format!("データID: {}", id),
        active: id % 2 == 0,  // 偶数IDならtrue、奇数ならfalse
        tags: vec![String::from("ID指定"), format!("ID-{}", id)],
    };
    
    // Json型でラップして返す
    Json(data)
}
