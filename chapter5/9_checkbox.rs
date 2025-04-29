impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.spacing();

            let msg = format!("checked = {}.", self.value);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.separator();

            let check_txt = egui::RichText::new("Checkbox").size(24.0);
            let check = egui::Checkbox::new(&mut self.value, check_txt);
            let _resp = ui.add(check);
        });
    }
}
