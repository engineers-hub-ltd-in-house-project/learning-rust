fn plot(ui: &mut egui::Ui) {
    let data = vec![
        egui::Pos2::new(50.0, 100.0),
        egui::Pos2::new(250.0, 100.0),
        egui::Pos2::new(75.0, 225.0),
        egui::Pos2::new(150.0, 50.0),
        egui::Pos2::new(225.0, 225.0),
    ];
    let stroke_1 = egui::Stroke::new(5.0, egui::Color32::RED);

    let mut shape_1 = eframe::epaint::PathShape::line(data, stroke_1);
    shape_1.closed = true;
    ui.painter().add(shape_1);
}
