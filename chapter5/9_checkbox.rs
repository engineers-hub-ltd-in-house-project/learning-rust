/*
 * チェックボックスの実装
 *
 * この例では、eguiフレームワークでチェックボックス（オン/オフを切り替えるUI要素）を
 * 実装する方法を示しています。
 *
 * チェックボックスの特徴：
 * 1. 真偽値（bool）の状態を持つ
 * 2. ユーザーがクリックすると状態がトグル（切り替え）される
 * 3. 状態に応じて処理を分岐させることができる
 * 4. 関連するUIを表示/非表示にするのに便利
 *
 * この例では、複数のチェックボックスを配置し、チェック状態に応じて
 * 追加のUIを表示する方法を示しています。
 */

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "チェックボックスの例",
        options,
        Box::new(|_cc| Box::new(CheckboxApp::default())),
    );
}

struct CheckboxApp {
    // チェックボックスの状態を格納するフィールド
    show_extra_options: bool,
    enable_feature: bool,
    receive_notifications: bool,
}

impl Default for CheckboxApp {
    fn default() -> Self {
        Self {
            show_extra_options: false,
            enable_feature: true,
            receive_notifications: false,
        }
    }
}

impl eframe::App for CheckboxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("チェックボックスのデモ");
            ui.add_space(10.0);

            // シンプルなチェックボックス
            ui.checkbox(&mut self.enable_feature, "機能を有効化");

            // もう一つのチェックボックス
            ui.checkbox(&mut self.receive_notifications, "通知を受け取る");

            // 追加オプションの表示/非表示を切り替えるチェックボックス
            ui.checkbox(&mut self.show_extra_options, "詳細設定を表示");

            // 追加オプションの表示部分
            if self.show_extra_options {
                // 詳細設定用のUIを分離するために枠線付きのセクションを追加
                ui.group(|ui| {
                    ui.label("詳細設定：");

                    // ラジオボタン風の選択肢
                    ui.radio_value(&mut self.enable_feature, true, "機能をオンにする");
                    ui.radio_value(&mut self.enable_feature, false, "機能をオフにする");

                    // さらに追加の設定
                    if self.enable_feature {
                        ui.label("機能が有効になっています！");

                        if self.receive_notifications {
                            ui.label("通知も有効です。");
                        }
                    }
                });
            }

            // 現在の設定状態を表示
            ui.separator();
            ui.label(format!("現在の設定状態:"));
            ui.label(format!(
                "機能の有効化: {}",
                if self.enable_feature {
                    "オン"
                } else {
                    "オフ"
                }
            ));
            ui.label(format!(
                "通知: {}",
                if self.receive_notifications {
                    "有効"
                } else {
                    "無効"
                }
            ));
        });
    }
}
