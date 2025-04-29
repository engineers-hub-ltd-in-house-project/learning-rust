# learning-rust

## プロジェクト概要
このプロジェクトはRust言語の基本的な学習用サンプルコードを集めたものです。Rustの様々な機能や概念を学ぶためのサンプルコードが含まれています。

## ディレクトリ構成
学習内容が整理されるよう、章ごとにディレクトリを分けています：
- chapter1: 基本構文
- chapter2: 変数、制御構文、データ構造
- chapter3: 所有権、借用、構造体
- chapter4: エラー処理、スレッド、ファイル操作
- chapter5: GUI開発 (egui)
- chapter6: Webサーバー開発 (axum)

## サンプルコードの構成

### 第1章: 基本構文
- chap1_1_hello_world: 基本的な「Hello, World!」
- chap1_2_sum: 数値の計算と表示
- chap1_3_hello_world_again: Hello Worldの別バージョン

### 第2章: 変数、制御構文、データ構造
- chap2_1_variables: 変数の宣言と使用
- chap2_2_variable_reassign_error: 変数の再代入エラー
- chap2_3_mut_variable: 可変変数の使用
- chap2_4_variable_shadowing: 変数のシャドーイング
- chap2_5_const: 定数の使用
- chap2_6_if_statement: if文による条件分岐
- chap2_7_elseif: else if文による条件分岐
- chap2_8_match: match式の基本
- chap2_9_match_or: match式での複数条件
- chap2_10_loop: loopによる繰り返し
- chap2_11_while: while文による繰り返し
- chap2_12_for: for文による繰り返し
- chap2_13_array_sum: 配列の要素の合計
- chap2_14_scope_error: スコープエラーの例
- chap2_15_array_sum: 配列要素合計の別バージョン
- chap2_16_tuple: タプルの使用方法
- chap2_17_tuple_destructure: タプルの分解
- chap2_18_tuple_const: 定数タプル
- chap2_19_vector: ベクタの基本
- chap2_20_vector_init: ベクタの初期化
- chap2_21_vector_modify: ベクタの修正
- chap2_22_hashmap: ハッシュマップの使用
- chap2_23_hashmap_value: ハッシュマップの値操作
- chap2_24_hashmap_loop: ハッシュマップのループ処理
- chap2_25_string: 文字列操作
- chap2_26_string_push: 文字列追加
- chap2_27_string_insert: 文字列挿入
- chap2_28_string_slice: 文字列スライス
- chap2_29_function: 関数の定義と使用
- chap2_30_function_calc: 関数による計算
- chap2_31_closure: クロージャの基本
- chap2_32_closure_capture: クロージャによる値のキャプチャ
- chap2_33_mutable_closure: 可変クロージャ
- chap2_34_mutable_closure_variable: クロージャ内の可変変数

### 第3章: 所有権、借用、構造体
- chap3_1_copy_str: 文字列リテラルのコピー
- chap3_2_move_string: Stringの所有権移動
- chap3_3_copy_number: 数値型のコピー
- chap3_4_function_move: 関数呼び出しでの所有権移動
- chap3_5_function_return: 関数からの所有権返却
- chap3_6_reference: 参照の使用
- chap3_7_reference_scope: 参照のスコープ
- chap3_8_mut_reference: 可変参照
- chap3_9_dereference: 参照外し
- chap3_10_slice: スライスの基本
- chap3_11_slice_reference: スライス参照
- chap3_12_slice_copy: スライスのコピー
- chap3_13_array_slice: 配列のスライス
- chap3_14_vector_slice: ベクターのスライス
- chap3_15_vector_slice_copy: ベクターのスライスコピー
- chap3_16_struct: 構造体の基本
- chap3_17_struct_constructor: 構造体のコンストラクタ関数
- chap3_18_struct_constructor_usage: コンストラクタの使い方
- chap3_19_tuple_struct: タプル構造体
- chap3_20_struct_reference: 構造体と参照
- chap3_21_struct_method: 構造体のメソッド
- chap3_22_static_method: 構造体の静的メソッド
- chap3_23_trait: トレイトの実装
- chap3_24_trait_function_arg: トレイトを引数に取る関数
- chap3_25_trait_function_ref: トレイト参照を引数に取る関数
- chap3_26_trait_function_ref_usage: トレイト参照関数の使用例
- chap3_27_trait_impl_print: トレイト実装を返す関数
- chap3_28_trait_impl_usage: トレイト実装関数の使用例
- chap3_29_trait_box: Box<dyn Trait>を返す関数
- chap3_30_trait_box_usage: Box<dyn Trait>関数の使用例
- chap3_31_debug_trait: Debugトレイトの使用
- chap3_32_default_trait: デフォルト実装を持つトレイト
- chap3_33_generic: ジェネリクス型
- chap3_34_generic_fn: ジェネリクス関数
- chap3_35_generic_impl: ジェネリック型のメソッド実装
- chap3_36_trait_box_setup: トレイトBoxの準備
- chap3_37_trait_box_usage: トレイトBox使用例
- chap3_38_trait_dyn: dynキーワードの使用
- chap3_39_enum_kind: 列挙型の定義と属性

### 第4章: エラー処理、スレッド、ファイル操作
- chap4_1_option: Option列挙型
- chap4_2_option_vec: Optionのベクタ
- chap4_3_option_random: ランダムなOption値
- chap4_4_option_match: Optionのmatch処理
- chap4_5_option_panic: Option値のパニック
- chap4_6_result: Result列挙型
- chap4_7_option_result: OptionからResultへの変換
- chap4_8_result_match: Resultのmatch処理
- chap4_9_error_kind: エラー種別の列挙型
- chap4_10_error_kind_usage: エラー種別の使用
- chap4_11_thread: スレッドの基本
- chap4_12_thread_sleep: スレッドのスリープ
- chap4_13_thread_loop: スレッドでのループ
- chap4_14_thread_wait: スレッドの待機
- chap4_15_thread_join: スレッドの結合
- chap4_16_thread_join_all: 複数スレッドの結合
- chap4_17_thread_move: move修飾子の使用
- chap4_18_thread_mutex: Mutexの使用
- chap4_19_thread_mutex_scope: Mutexのスコープ
- chap4_20_deadlock: デッドロックの例
- chap4_21_thread_channel: チャネルの使用
- chap4_22_thread_dual_channel: 双方向チャネル
- chap4_23_thread_sync_channel: 同期チャネル
- chap4_24_file_read: ファイル読み込み
- chap4_25_file_read_error: ファイル読み込みエラー処理
- chap4_26_file_write: ファイル書き込み
- chap4_27_file_append: ファイル追記
- chap4_28_dir_read: ディレクトリ読み込み
- chap4_29_dir_file_type: ファイルタイプ判定
- chap4_30_file_backup: ファイルバックアップ

### 第5章: GUI開発 (egui)
- chap5_1_eframe_setup: eframeの基本設定
- chap5_3_central_panel: 中央パネルの表示
- chap5_4_light_theme: ライトテーマの設定
- chap5_5_label: ラベルの使用
- chap5_6_label_style: ラベルのスタイリング
- chap5_7_button: ボタンの実装
- chap5_8_checkbox_setup: チェックボックスの準備
- chap5_9_checkbox: チェックボックスの実装
- chap5_10_radio_enum: ラジオボタン用の列挙型
- chap5_11_radio_setup: ラジオボタンの準備
- chap5_12_radio: ラジオボタンの実装
- chap5_13_slider_setup: スライダーの準備
- chap5_14_slider: スライダーの実装
- chap5_15_drag_value: ドラッグ可能な値
- chap5_16_selectable_enum: 選択可能項目の列挙型
- chap5_17_selectable_setup: 選択可能項目の準備
- chap5_18_selectable: 選択可能項目の実装
- chap5_19_textEdit_setup: テキスト編集の準備
- chap5_20_textEdit: テキスト編集機能
- chap5_21_textEdit_changed: テキスト変更検知
- chap5_22_painter_setup: ペインターの準備
- chap5_23_painter_rect: 矩形描画
- chap5_24_painter_rect_vars: 変数を使った矩形描画
- chap5_25_painter_circle: 円描画
- chap5_26_painter_line: 線描画
- chap5_27_painter_text: テキスト描画
- chap5_28_painter_path: パス描画
- chap5_29_painter_click_setup: クリックイベントの準備
- chap5_30_painter_click: クリックイベントの処理

### 第6章: Webサーバー開発 (axum)
- chap6_1_axum_hello: Axumの基本的なサーバー
- chap6_2_axum_handler: ハンドラー関数の実装
- chap6_4_axum_param_route: パラメータ付きルート設定
- chap6_5_axum_param: パラメータの処理
- chap6_6_axum_multi_param_route: 複数パラメータルート
- chap6_7_axum_multi_param: 複数パラメータの処理
- chap6_8_axum_query_route: クエリパラメータルート
- chap6_9_axum_query: クエリパラメータの処理
- chap6_10_cargo_serde: Serde依存関係設定
- chap6_12_axum_json_route: JSONレスポンスルート
- chap6_13_mydata_struct: JSONデータ用構造体
- chap6_14_axum_json: JSONレスポンス実装
- chap6_15_cargo_tera: Tera依存関係設定
- chap6_16_tera_template: Teraテンプレート定義
- chap6_17_axum_tera: Axum+Teraの基本設定
- chap6_18_axum_tera_handler: Teraテンプレートレンダリング
- chap6_19_tera_form_template: フォーム用テンプレート
- chap6_20_axum_form_route: フォーム処理ルート
- chap6_21_serde_use: Serdeインポート
- chap6_22_form_struct: フォームデータ構造体
- chap6_23_form_handler: フォームデータ処理

## 使用方法

### サンプルコードの実行
各サンプルは以下のコマンドで実行できます：

```
cargo run --example サンプル名
```

例:
```
cargo run --example chap1_1_hello_world
cargo run --example chap2_10_loop
cargo run --example chap3_16_struct
cargo run --example chap5_7_button
```

### 実行サンプルと結果

#### Hello World (chap1_1_hello_world)

```rust
fn main() {
  println!("Hello, world!");
}
```

実行結果:
```
Hello, world!
```

#### 変数と計算 (chap2_1_variables)

```rust
fn main() {
  let x = 100;
  let y:i64 = 200;
  let z = x + y;
  println!("{} + {} = {}", x, y, z);
}
```

実行結果:
```
100 + 200 = 300
```

#### 可変変数 (chap2_3_mut_variable)

```rust
fn main() {
  let x = 123;
  let y = 45;
  let mut z = x + y;
  println!("{} + {} = {}", x, y, z);
  z = x - y;
  println!("{} - {} = {}", x, y, z);
}
```

実行結果:
```
123 + 45 = 168
123 - 45 = 78
```

#### forループ (chap2_12_for)

```rust
fn main() {
  let max = 100;
  let mut ans = 0;
  for item in 1..=max {
    ans += item;
  }
  println!("1から{}までの合計は、{} です。", max, ans);
}
```

実行結果:
```
1から100までの合計は、5050 です。
```

#### 構造体とメソッド (chap3_21_struct_method)

```rust
struct Person {
  name:String,
  mail:String,
  age:i32
}

fn person(name:String, mail:String, age:i32)-> Person {
  Person {name, mail, age}
}

impl Person {
  fn print(&self) {
    println!("{}<{}>({}).", self.name, self.mail, self.age);
  }
}

fn main() {
  let taro = person(
    String::from("Taro"),
    String::from("taro@yamada"),
    39
  );
  let hanako = person(
    String::from("Hanako"),
    String::from("hanako@flower"),
    28
  );
  taro.print();
  hanako.print();
}
```

実行結果:
```
Taro<taro@yamada>(39).
Hanako<hanako@flower>(28).
```

#### ボタン付きGUI (chap5_7_button)

```rust
fn main() {
  let mut native_options = eframe::NativeOptions::default();
  native_options.default_theme = eframe::Theme::Light;
  native_options.initial_window_size = Some(egui::Vec2 {x:400.0, y:200.0});

  let _ = eframe::run_native("My egui App", native_options,
    Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
  pub value:usize,
}

impl Default for MyEguiApp {
  fn default() -> MyEguiApp {
    MyEguiApp{ value:0 }
  }
}

impl MyEguiApp {
  fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self::default()
  }
}

impl eframe::App for MyEguiApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Hello World!");
      
      ui.spacing();

      let msg = format!("click {} times.", self.value);
      let label_txt = egui::RichText::new(msg).size(32.0);
      let label = egui::Label::new(label_txt);
      ui.add(label);

      ui.separator();

      let btn_txt = egui::RichText::new("Click!").font(egui::FontId::proportional(24.0));
      let btn = egui::Button::new(btn_txt);
      let resp = ui.add_sized(egui::Vec2 {x:150.0, y:40.0}, btn);
      if resp.clicked() {
        self.value += 1;
      }
    });
  }
}
```

実行結果:
クリック可能なボタンを持つGUIウィンドウが表示されます。

#### Webサーバー (chap6_1_axum_hello)

```rust
#[tokio::main]
async fn main() {
  let app = axum::Router::new()
    .route("/", axum::routing::get(|| async { "Hello, World!" }));

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
```

実行結果:
サーバーが起動し、ブラウザで http://127.0.0.1:3000 にアクセスすると「Hello, World!」と表示されます。

### 注意事項
- 一部のコードサンプル（特にchap3_20_struct_referenceなど）は、そのままでは実行時にコンパイルエラーになる可能性があります。これらのサンプルはRustの概念を示すためのものであり、実行するには追加のコード（ライフタイム注釈など）が必要な場合があります。
- ファイル操作のサンプル（chap4_24_file_read.rsなど）を実行する場合、必要なファイルを事前に作成するか、コードを適宜修正して使用してください。
- GUIサンプル（chapter5）やWebサーバーサンプル（chapter6）の実行には、それぞれ必要な依存関係をCargo.tomlに記述する必要があります。

## 依存関係
主な依存関係は以下の通りです：

```toml
[dependencies]
rand = "0.8.5"                                # 乱数生成
eframe = "0.21.0"                             # GUIフレームワーク
egui = "0.21.0"                               # GUIライブラリ
axum = "0.6.9"                                # Webフレームワーク
hyper = { version = "0.14", features = ["full"] } # HTTPサーバー
tokio = { version = "1.25", features = ["full"] } # 非同期ランタイム
tower = { version = "0.4", features = ["full"] }  # HTTPサービス抽象化
serde = { version = "1.0", features = ["derive"] } # シリアライズ/デシリアライズ
serde_json = "1.0"                            # JSONサポート
axum-template = "0.14.0"                      # テンプレートエンジン連携
tera = "1.17.1"                               # テンプレートエンジン
```

## Git操作

リポジトリを初期化してGitHubにプッシュするには以下のコマンドを実行します:

```bash
git add .
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/engineers-hub-ltd-in-house-project/learning-rust.git
git push -u origin main
``` 