/*
 * ボタンの実装
 *
 * この例では、eguiフレームワークでボタンを実装し、そのクリックイベントを処理する方法を示しています。
 * ボタンはGUIアプリケーションの基本的な要素で、ユーザー操作の入り口となります。
 *
 * このサンプルで学べること：
 * 1. ボタンの作成と表示
 * 2. ボタンのスタイル設定（サイズ、フォント）
 * 3. クリックイベントの処理
 * 4. クリック回数を保持する状態管理
 * 5. クリックに応じた表示内容の更新
 *
 * ボタンは、ui.add()メソッドでUIに追加し、返される応答（Response）オブジェクトの
 * clicked()メソッドでクリックイベントを検出できます。
 */

use eframe::egui;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.default_theme = eframe::Theme::Light;
    native_options.initial_window_size = Some(egui::Vec2 { x: 400.0, y: 200.0 });

    let _ = eframe::run_native(
        "Button Example",
        native_options,
        Box::new(|cc| Box::new(ButtonApp::new(cc))),
    );
}

struct ButtonApp {
    count: usize,
}

impl Default for ButtonApp {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl ButtonApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ButtonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Button Demo");

            ui.spacing();

            let message = format!("Click count: {}", self.count);
            let label_text = egui::RichText::new(message).size(32.0);
            ui.label(label_text);

            ui.separator();

            let button_text =
                egui::RichText::new("Click Me!").font(egui::FontId::proportional(24.0));
            let button = egui::Button::new(button_text);

            let response = ui.add_sized(egui::Vec2 { x: 150.0, y: 40.0 }, button);

            if response.clicked() {
                self.count += 1;
            }
        });
    }
}
