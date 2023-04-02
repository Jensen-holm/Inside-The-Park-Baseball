use gtk4::{Align, Application, ApplicationWindow, Button, Box, Label, CssProvider, StyleContext, Orientation, Stack, StackTransitionType};
use gtk4::gdk::Display;
use gtk4::glib::clone;
use gtk4::prelude::*;


pub(crate) fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}


pub(crate) fn home_screen(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Inside The Park Baseball '23")
        .default_width(1000)
        .default_height(600)
        .build();

    let home_stack: Stack = Stack::builder()
        .transition_type(StackTransitionType::Crossfade)
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
    page1_button.connect_clicked(clone!(@weak stack => move |_| {
        stack.set_visible_child_name("page2");
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

    home_stack.add_named(&home_box, Some("Home Page"));

    window.set_child(Some(&home_stack));
    window.show();
}
