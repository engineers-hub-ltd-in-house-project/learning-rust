# learning-rust

## プロジェクト概要
このプロジェクトはRust言語の基本的な学習用サンプルコードを集めたものです。Rustの様々な機能や概念を学ぶためのサンプルコードが含まれています。

## ディレクトリ構成
学習内容が整理されるよう、章ごとにディレクトリを分けています：
- chapter1: 基本構文
- chapter2: 変数、制御構文、データ構造
- chapter3: 所有権、借用、構造体
- chapter4: エラー処理、スレッド、ファイル操作

## サンプルコードの構成

### 第1章: 基本構文
- chap1_1_hello_world: 基本的な「Hello, World!」
- chap1_2_sum: 数値の計算と表示
- chap1_3_hello_world_again: Hello Worldの別バージョン

### 第2章: 変数、制御構文、データ構造
- chap2_1_variables: 変数の宣言と使用
- chap2_3_mut_variable: 可変変数の使用
- chap2_6_if_statement: if文による条件分岐
- chap2_8_match: match式の基本
- chap2_9_match_or: match式での複数条件
- chap2_10_loop: loopによる繰り返し
- chap2_11_while: while文による繰り返し
- chap2_12_for: for文による繰り返し
- chap2_13_array_sum: 配列の要素の合計
- chap2_16_tuple: タプルの使用方法
- chap2_22_hashmap: ハッシュマップの使用
- chap2_25_string: 文字列操作
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
- chap3_33_generic: ジェネリクス型

### 第4章: エラー処理、スレッド、ファイル操作
- chap4_1_option: Option列挙型
- chap4_6_result: Result列挙型
- chap4_11_thread: スレッドの基本
- chap4_24_file_read: ファイル読み込み

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

#### スレッド (chap4_11_thread)

```rust
use std::thread;

fn main() {
  thread::spawn(|| {
    println!("Thread:Start!");
    println!("Thread:End.");
  });

  println!("Main:Start!");
  println!("Main:End.");
}
```

実行結果 (実行ごとに出力順序が変わる可能性があります):
```
Main:Start!
Main:End.
Thread:Start!
Thread:End.
```

### 注意事項
- 一部のコードサンプル（特にchap3_20_struct_referenceなど）は、そのままでは実行時にコンパイルエラーになる可能性があります。これらのサンプルはRustの概念を示すためのものであり、実行するには追加のコード（ライフタイム注釈など）が必要な場合があります。
- ファイル操作のサンプル（chap4_24_file_read.rsなど）を実行する場合、必要なファイルを事前に作成するか、コードを適宜修正して使用してください。

## Git操作

リポジトリを初期化してGitHubにプッシュするには以下のコマンドを実行します:

```bash
git add .
git commit -m "first commit"
git branch -M main
git remote add origin https://github.com/engineers-hub-ltd-in-house-project/learning-rust.git
git push -u origin main
``` 