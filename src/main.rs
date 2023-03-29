pub mod gui;

use gtk::prelude::*;

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.Jensen.BaseballSim")
        .build();

    app.connect_activate(gui::on_activate);
    app.run();
}
