struct MyEguiApp {
    pub value: MyItem,
}

impl Default for MyEguiApp {
    fn default() -> MyEguiApp {
        MyEguiApp {
            value: MyItem::First,
        }
    }
}
