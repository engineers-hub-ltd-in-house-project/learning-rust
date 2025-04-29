/*
 * Axumのクエリパラメータ処理
 *
 * この例では、Axumフレームワークを使用してURLのクエリパラメータを
 * 処理する方法を示しています。クエリパラメータは、URLの末尾に`?key=value`の形式で
 * 付加される値であり、Webアプリケーションでデータを送信する一般的な方法です。
 * 
 * クエリパラメータの特徴：
 * 1. URLに付加される値（例: /search?q=rust&page=2）
 * 2. 複数のパラメータを&で連結できる
 * 3. 任意のパラメータで、存在しなくても処理できる必要がある
 * 4. axum::extract::Query抽出器を使用して取得する
 * 
 * この例では、クエリパラメータを取得して、その内容をレスポンスに含める
 * ハンドラー関数を実装しています。
 */

// クエリパラメータを処理するハンドラー関数
async fn handler_query(query: axum::extract::Query<std::collections::HashMap<String, String>>) -> String {
    // クエリパラメータをHashMapとして取得
    let params = query.0;
    
    // パラメータの数をチェック
    if params.is_empty() {
        return "クエリパラメータがありません。例: ?name=value".to_string();
    }
    
    // 結果文字列を構築
    let mut result = "クエリパラメータ:\n".to_string();
    
    // 各パラメータをループして結果に追加
    for (key, value) in params {
        result.push_str(&format!("- {}: {}\n", key, value));
    }
    
    result
}
