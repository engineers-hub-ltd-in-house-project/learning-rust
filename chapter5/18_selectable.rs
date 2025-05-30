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

            ui.horizontal(|ui| {
                let label_1 = egui::RichText::new("First").size(24.0);
                if ui
                    .add(egui::SelectableLabel::new(
                        self.value == MyItem::First,
                        label_1,
                    ))
                    .clicked()
                {
                    self.value = MyItem::First
                }
                let label_2 = egui::RichText::new("Second").size(24.0);
                if ui
                    .add(egui::SelectableLabel::new(
                        self.value == MyItem::Second,
                        label_2,
                    ))
                    .clicked()
                {
                    self.value = MyItem::Second
                }
                let label_3 = egui::RichText::new("Third").size(24.0);
                if ui
                    .add(egui::SelectableLabel::new(
                        self.value == MyItem::Third,
                        label_3,
                    ))
                    .clicked()
                {
                    self.value = MyItem::Third
                }
            });
        });
    }
}
