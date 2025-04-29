fn plot(ui: &mut egui::Ui) {
    let pos_1 = egui::Pos2::new(50.0, 50.0);
    let pos_2 = egui::Pos2::new(200.0, 200.0);
    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::RED);
    let stroke_2 = egui::Stroke::new(5.0, egui::Color32::GREEN);
    ui.painter()
        .vline(50.0, std::ops::RangeInclusive::new(50.0, 200.0), stroke_1);
    ui.painter()
        .hline(std::ops::RangeInclusive::new(50.0, 200.0), 50.0, stroke_1);
    ui.painter().line_segment([pos_1, pos_2], stroke_2);
}
