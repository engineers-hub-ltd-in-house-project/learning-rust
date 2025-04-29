/*
 * 中央パネルの表示
 *
 * この例では、eguiフレームワークの中央パネル（CentralPanel）を使用して
 * GUIアプリケーションのメインコンテンツを表示する方法を示しています。
 *
 * 中央パネルの特徴：
 * 1. 画面の中央部分を占めるメインコンテンツ領域
 * 2. サイドパネル、トップパネルなどを配置した後の残りのスペースを自動的に使用
 * 3. シンプルなレイアウトを素早く作成できる
 *
 * この例では、CentralPanelを使用して「Hello, World!」というテキストを表示しています。
 */

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "中央パネルの例",
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
        // 中央パネルを作成し、その中にコンテンツを表示
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
            ui.label("これは中央パネルに表示されているテキストです。");
        });
    }
}
