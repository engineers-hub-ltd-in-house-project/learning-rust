/*
 * フォームデータ処理ハンドラー
 *
 * この例では、Axumフレームワークを使用してHTMLフォームから送信されたデータを
 * 処理する方法を示しています。FormメソッドでPOSTリクエストを受け取り、
 * HTMLテンプレートでレスポンスを生成します。
 *
 * このサンプルで学べること：
 * 1. AxumでのFormデータの取得と型変換
 * 2. Teraテンプレートエンジンとの連携
 * 3. フォームデータを使用したHTMLレスポンスの生成
 * 4. 非同期ハンドラー関数の実装
 *
 * フォームから送信された名前とメールアドレスを取得し、
 * それらを使用してテンプレートにデータを渡し、HTMLページをレンダリングしています。
 */

use crate::Myform;
use axum::extract::Form;
use axum::response::Html;
use tera::Tera;

// POSTリクエストを処理するハンドラー関数
pub async fn handle_post(Form(form): Form<Myform>) -> Html<String> {
    // 送信されたフォームデータからメッセージを作成
    let message = format!("{}さん({})", form.name, form.mail);

    // テンプレートエンジンをインスタンス化
    let mut tera = Tera::default();
    tera.add_raw_template("index.html", include_str!("../templates/index.html"))
        .unwrap();

    // テンプレートに渡すコンテキスト（データ）を作成
    let mut context = tera::Context::new();
    context.insert("title", "フォーム送信完了");
    context.insert("message", &message);

    // テンプレートをレンダリングしてHTMLレスポンスを返す
    Html(tera.render("index.html", &context).unwrap())
}
