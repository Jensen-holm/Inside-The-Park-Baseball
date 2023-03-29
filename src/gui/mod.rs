mod widgets;

use gtk::prelude::*;

fn on_activate(app: &gtk::Application) {
    let win = gtk::ApplicationWindow::new(app);
    win.set_size_request(1200, 800);
    win.present();
}

pub(crate) fn main() {
    let app = gtk::Application::builder()
        .application_id("jensen")
        .build();

    app.connect_activate(on_activate);
    app.run();
}
