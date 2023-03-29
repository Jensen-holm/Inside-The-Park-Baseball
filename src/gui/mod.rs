use gtk4::{Align, Application, ApplicationWindow, Button};
use gtk4::traits::{GtkWindowExt, WidgetExt};


pub(crate) fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Baseball Simulator Hub")
        .default_width(1200)
        .default_height(800)
        .build();

    let button = Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_top(10)
        .halign(Align::Center)
        .valign(Align::Center)
        .label("Click Me!")
        .build();

    window.set_child(Some(&button));
    window.show();
}
