fn main() {
    // これはサンプルコードの一部です
    // 実際にはMyEguiAppのupdate関数内で使用されるコードの一部です
    let mut value = 0;
    let ui = &mut egui::Ui::__for_test();

    let drg = egui::DragValue::new(&mut value).speed(1);
    ui.add(drg);
}
