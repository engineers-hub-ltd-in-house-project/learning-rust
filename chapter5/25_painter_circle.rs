fn plot(ui: &mut egui::Ui) {
    let pos_1 = egui::Pos2::new(100.0, 100.0);
    ui.painter().circle_filled(pos_1, 50.0, egui::Color32::RED);
    let pos_2 = egui::Pos2::new(150.0, 150.0);
    let stroke_2 = egui::Stroke::from((10.0, egui::Color32::GREEN));
    ui.painter().circle_stroke(pos_2, 50.0, stroke_2);
}
