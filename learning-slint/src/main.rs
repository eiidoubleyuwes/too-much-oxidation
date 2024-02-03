slint::slint! {
     component Memorytile inherits Rectangle{
         width: 64px;
         height: 64px;
         background: #3960D5;  
     }export component MainWindow inherits Window {
        Text {
            text: "Mnjala Baraka on top";
            color: green;
        }
        Memorytile{}
    }
    

    }
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
