struct MyEguiApp {
    pub value: usize,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp { value: 0 }
    }
}
