/*
 * Axumの基本的なサーバー
 *
 * この例では、Rustの軽量なWebフレームワークであるAxumを使用して、
 * シンプルなHTTPサーバーを構築しています。
 *
 * この例で学べる内容：
 * 1. Tokioランタイムの設定（非同期処理の基盤）
 * 2. Axumルーターの基本設定
 * 3. HTTPエンドポイントの定義（ルーティング）
 * 4. シンプルなレスポンスの返却
 * 5. サーバーの起動と待機
 *
 * この例ではルートパス（"/"）へのGETリクエストに対して
 * "Hello, World!"という文字列を返すだけのシンプルなサーバーを構築しています。
 * サーバーは127.0.0.1:3000（localhost:3000）で起動します。
 */

// Tokioの非同期ランタイムを使用することを宣言
#[tokio::main]
async fn main() {
    // ルーターの設定 - ルートパスへのGETリクエストに対するハンドラーを定義
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    // サーバーをバインドして起動
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
