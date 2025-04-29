/*
 * フォームデータ構造体
 *
 * この例では、HTMLフォームからのデータを受け取るためのRust構造体を定義しています。
 * SerializeとDeserializeのトレイトを実装することで、フォームデータとRustの構造体との
 * 自動的な変換が可能になります。
 * 
 * 特徴:
 * 1. 構造体のフィールド名はフォームのフィールド名と一致する必要がある
 * 2. #[derive(Serialize, Deserialize)]アトリビュートにより自動的に変換機能が実装される
 * 3. axum::extract::Formエクストラクタと組み合わせて使用する
 * 
 * この構造体は、nameとmailの2つのフィールドを持つフォームデータを受け取るために使用されます。
 */

#[derive(Serialize, Deserialize)]
struct Myform {
    name: String,
    mail: String,
}
