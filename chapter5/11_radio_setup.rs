struct MyEguiApp {
    pub value: RadioValue,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            value: RadioValue::First,
        }
    }
}
