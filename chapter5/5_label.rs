/*
 * ラベルの使用
 *
 * この例では、eguiフレームワークでラベル（テキスト表示）を実装する方法を示しています。
 * ラベルはGUIアプリケーションで最も基本的なUIコンポーネントの一つであり、
 * 静的テキストを表示するために使用されます。
 *
 * この例で学べること：
 * 1. ラベルの基本的な使い方
 * 2. 様々な方法でのラベル作成
 * 3. ラベルのスタイリング（RichText）
 *
 * eguiではui.labelメソッドを使用する方法と、Label::newを使ってラベルを
 * 作成する方法があります。また、RichTextを使うとテキストのスタイルをカスタマイズできます。
 */

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ラベルの例",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 見出し（大きなラベル）
            ui.heading("ラベルの例");

            ui.add_space(10.0); // スペースを追加

            // 通常のラベル
            ui.label("これは通常のラベルです");

            // 別の方法でラベルを作成
            let my_label = egui::Label::new("これはLabel::newで作成したラベルです");
            ui.add(my_label);

            // RichTextを使ったスタイル付きラベル
            let styled_text = egui::RichText::new("スタイル付きテキスト")
                .size(24.0)
                .color(egui::Color32::from_rgb(255, 100, 100))
                .italics();
            ui.label(styled_text);
        });
    }
}
