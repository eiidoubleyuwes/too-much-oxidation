fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "Mnjala Baraka on top";
            color: green;
        }
    }
}
