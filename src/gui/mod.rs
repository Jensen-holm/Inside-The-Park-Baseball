use gtk4::{Align, Application, ApplicationWindow, Button, Box, Label};
use gtk4::traits::{BoxExt, GtkWindowExt, WidgetExt};


pub(crate) fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Baseball Simulator Hub")
        .default_width(1000)
        .default_height(600)
        .build();

    let option_box: Box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let home_label: Label = Label::builder()
        .label("IOTP Baseball")
        .margin_top(100)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    let start_new: Button = Button::builder()
        .margin_top(200)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .valign(Align::Center)
        .halign(Align::Center)
        .label("Start New Game")
        .build();

    let load_save: Button = Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .valign(Align::Center)
        .halign(Align::Center)
        .label("Load Save Game")
        .build();

    option_box.append(&home_label);
    option_box.append(&start_new);
    option_box.append(&load_save);

    window.set_child(Some(&option_box));
    window.show();
}

