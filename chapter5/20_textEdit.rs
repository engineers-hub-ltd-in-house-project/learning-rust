impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.spacing();

            let msg = format!("Title:\"{}\"\nContent:[{}]", self.message, self.content);
            let label_txt = egui::RichText::new(msg).font(egui::FontId::proportional(24.0));
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.separator();

            let te_sl = egui::TextEdit::singleline(&mut self.message)
                .font(egui::FontId::proportional(20.0));
            ui.add(te_sl);
            let te_ml =
                egui::TextEdit::multiline(&mut self.content).font(egui::FontId::proportional(20.0));
            ui.add(te_ml);
        });
    }
}
