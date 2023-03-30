use gtk4::{Align, Application, ApplicationWindow, Button, Box};
use gtk4::traits::{BoxExt, GtkWindowExt, WidgetExt};


pub(crate) fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Baseball Simulator Hub")
        .default_width(1200)
        .default_height(800)
        .build();

    let option_box: Box = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let start_new: Button = home_screen_button(
        "Start New",
        Align::Start,
        Align::Start,
    );

    let load_save: Button = home_screen_button(
        "Load Save",
        Align::Start,
        Align::Start,
    );

    option_box.append(&start_new);
    option_box.append(&load_save);

    window.set_child(Some(&option_box));
    window.show();
}


fn home_screen_button(
    txt: &str,
    h_alignment: Align,
    v_alignment: Align,
) -> Button {
    return Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_bottom(10)
        .halign(h_alignment)
        .valign(v_alignment)
        .label(txt)
        .build();
}
