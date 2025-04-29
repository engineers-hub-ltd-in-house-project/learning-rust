impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let label_txt = egui::RichText::new("This is sample message.")
                .font(egui::FontId::proportional(32.0));
            let label = egui::Label::new(label_txt);
            ui.add(label);
        });
    }
}
