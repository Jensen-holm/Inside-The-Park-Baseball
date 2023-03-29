use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Button};

pub(crate) fn on_activate(app: &gtk::Application) {
    let win = gtk::ApplicationWindow::new(app);
    win.set_size_request(1200, 800);

    // button that closes window
    let button = Button::with_label("Hello World!");
    button.connect_clicked(clone!(@weak win => move |_| win.close()));

    win.set_child(Some(&button));
    win.present();
}
