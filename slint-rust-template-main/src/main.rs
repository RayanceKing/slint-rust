slint::include_modules!();
pub fn main() {
    let main_window = MainWindow::new().expect("Failed to create window");
    main_window.run();
}
