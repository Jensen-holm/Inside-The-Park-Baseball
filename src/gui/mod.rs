use gtk4::{Align, Application, ApplicationWindow, Button, Box, Label, CssProvider, StyleContext, Image, Overlay, Orientation};
use gtk4::gdk::Display;
use gtk4::prelude::{BoxExt, GtkWindowExt, WidgetExt};

pub(crate) fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub(crate) fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Baseball Simulator Hub")
        .default_width(1000)
        .default_height(600)
        .build();

    let background_image: Image = Image::from_file("src/gui/assets/image.png");
    background_image.set_halign(Align::Center);
    background_image.set_valign(Align::Center);
    background_image.set_size_request(1000, 600);

    // Create an Overlay widget and add the Image widget as a child
    let overlay = Overlay::new();
    overlay.add_overlay(&background_image);

    let option_box: Box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let home_label: Label = Label::builder()
        .label("IOTP Baseball")
        .margin_top(100)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();
    home_label.add_css_class("home_title");
    option_box.append(&home_label);

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
    option_box.append(&start_new);

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
    option_box.append(&load_save);

    // Add the option_box to the Overlay
    overlay.add_overlay(&option_box);

    // Set the Overlay as the child of the window
    window.set_child(Some(&overlay));

    window.show();
}
