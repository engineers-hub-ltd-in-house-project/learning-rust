struct MyEguiApp {
    pub value: bool,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp { value: true }
    }
}
