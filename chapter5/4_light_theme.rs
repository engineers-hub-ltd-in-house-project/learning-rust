/*
 * ライトテーマの設定
 *
 * この例では、eguiフレームワークでアプリケーションのテーマをライトモード（明るい配色）に
 * 設定する方法を示しています。
 * 
 * 主な内容：
 * 1. NativeOptionsを使ったアプリケーション設定のカスタマイズ
 * 2. テーマの設定（ライトモード/ダークモード）
 * 3. ウィンドウサイズの初期設定
 * 
 * eguiのデフォルトはダークテーマですが、NativeOptionsのdefault_themeプロパティを
 * 変更することで、アプリケーションの見た目を明るい色調に変更できます。
 */

use eframe::egui;

fn main() {
    // カスタマイズしたアプリケーション設定を作成
    let mut options = eframe::NativeOptions::default();
    
    // テーマをライトモードに設定
    options.default_theme = eframe::Theme::Light;
    
    // 初期ウィンドウサイズを設定
    options.initial_window_size = Some(egui::Vec2 { x: 400.0, y: 300.0 });
    
    // アプリケーションの起動
    eframe::run_native(
        "ライトテーマの例",
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
            ui.heading("ライトテーマのアプリケーション");
            ui.label("こちらは明るい配色のUIです");
            ui.separator();
            ui.label("背景色や文字色がライトモード向けに最適化されています");
        });
    }
}
