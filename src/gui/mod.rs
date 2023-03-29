extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};


pub(crate) fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello, World!");
    window.set_default_size(350, 70);

    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Hello, World!");
    });

    window.add(&button);

    window.show_all();

    gtk::main();
}

