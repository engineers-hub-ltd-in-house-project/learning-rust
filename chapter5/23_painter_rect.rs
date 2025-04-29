fn plot(ui: &mut egui::Ui) {
    ui.painter().rect_filled(
        egui::Rect::from_min_max(egui::Pos2::new(50.0, 50.0), egui::Pos2::new(150.0, 150.0)),
        egui::Rounding::same(20.0),
        egui::Color32::RED,
    );
    ui.painter().rect_stroke(
        egui::Rect::from_min_max(egui::Pos2::new(100.0, 100.0), egui::Pos2::new(200.0, 200.0)),
        egui::Rounding::none(),
        egui::Stroke::new(10.0, egui::Color32::GREEN),
    );
}
