impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.spacing();

            let msg = format!("value = {:?}.", self.value);
            let label_txt = egui::RichText::new(msg).size(28.0);
            let label = egui::Label::new(label_txt);
            ui.add(label);

            ui.separator();

            let sldr = egui::Slider::new(&mut self.value, 0..=100);
            ui.add(sldr);
        });
    }
}
