fn plot(ui: &mut egui::Ui) {
    let rect_1 =
        egui::Rect::from_min_max(egui::Pos2::new(50.0, 50.0), egui::Pos2::new(150.0, 150.0));
    let round_1 = egui::Rounding::same(20.0);

    ui.painter()
        .rect_filled(rect_1, round_1, egui::Color32::RED);

    let rect_2 =
        egui::Rect::from_min_max(egui::Pos2::new(100.0, 100.0), egui::Pos2::new(200.0, 200.0));
    let round_2 = egui::Rounding::none();
    let stroke_2 = egui::Stroke::from((10.0, egui::Color32::GREEN));

    ui.painter().rect_stroke(rect_2, round_2, stroke_2);
}
