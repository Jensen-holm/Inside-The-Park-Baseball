use gtk4::{Align, Application, ApplicationWindow, Button, Box, Label, CssProvider, StyleContext, Orientation};
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::glib::clone;
use std::rc::Rc;

pub(crate) fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub(crate) fn build_home(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Inside The Park Baseball '23")
        .default_width(1000)
        .default_height(600)
        .build();

    let home_box: Box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let home_label: Label = Label::builder()
        .label("IOTP Baseball")
        .margin_top(100)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    home_label.add_css_class("home_title");
    home_box.append(&home_label);

    let app_rc = Rc::new(app.clone()); // Wrap app in an Rc pointer

    let start_new: Button = Button::builder()
        .margin_top(200)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .valign(Align::Center)
        .halign(Align::Center)
        .label("Start New Game")
        .build();
    start_new.add_css_class("home_button");

    // Clone the Rc pointer and move it into the closure
    start_new.connect_clicked(clone!(@strong app_rc => move |_| {
        let new_win = new_window(&app_rc); // Use the cloned Rc pointer
        new_win.show();
    }));

    home_box.append(&start_new);

    let load_save: Button = Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .valign(Align::Center)
        .halign(Align::Center)
        .label("Load Save Game")
        .build();
    load_save.add_css_class("home_button");
    home_box.append(&load_save);

    window.set_child(Some(&home_box));
    window.show();
}


pub(crate) fn new_window(app: &Application) -> ApplicationWindow {
    return ApplicationWindow::builder()
        .application(app)
        .build();

}
