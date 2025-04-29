struct MyEguiApp {
    pub message: String,
    pub content: String,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            message: String::from("Hello"),
            content: String::from("This is content."),
        }
    }
}
