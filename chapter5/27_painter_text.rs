fn plot(ui: &mut egui::Ui) {
    ui.painter().text(
        egui::Pos2 { x: 50.0, y: 50.0 },
        egui::Align2::LEFT_CENTER,
        "Hello!",
        egui::FontId::proportional(24.0),
        egui::Color32::RED,
    );
    ui.painter().text(
        egui::Pos2 { x: 50.0, y: 100.0 },
        egui::Align2::LEFT_CENTER,
        "Sample Message.",
        egui::FontId::proportional(36.0),
        egui::Color32::BLUE,
    );
}
