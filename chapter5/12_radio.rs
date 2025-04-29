impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.spacing();

            let msg = format!("checked = {:?}.", self.value);
            let label_txt = egui::RichText::new(msg).size(32.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.separator();

            let label_1 = egui::RichText::new("First").size(24.0);
            let label_2 = egui::RichText::new("Second").size(24.0);
            let label_3 = egui::RichText::new("Third").size(24.0);
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.value, RadioValue::First, label_1);
                ui.radio_value(&mut self.value, RadioValue::Second, label_2);
                ui.radio_value(&mut self.value, RadioValue::Third, label_3);
            });
        });
    }
}
