struct MyEguiApp {
    click_pos: Vec<egui::Pos2>,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp { click_pos: vec![] }
    }
}
