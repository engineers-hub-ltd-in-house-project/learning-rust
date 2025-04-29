impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let resp = ui.allocate_response(egui::vec2(400.0, 300.0), egui::Sense::click());
            if resp.clicked() {
                let p = resp.interact_pointer_pos().unwrap();
                self.click_pos.push(p);
            }
            plot(ui, &self.click_pos);
        });
    }
}

fn plot(ui: &mut egui::Ui, pos: &Vec<egui::Pos2>) {
    for p in pos {
        ui.painter().circle_filled(
            *p,
            25.0,
            egui::Color32::from_rgba_premultiplied(255, 0, 0, 100),
        );
    }
}
