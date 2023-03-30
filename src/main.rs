mod gui;

use gtk4::{glib, Application};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};

fn main() -> glib::ExitCode {
    let application = Application::new(
        Some("com.github.Jensen-holm.bbref_sim_app"),
        Default::default(),
    );

    application.connect_startup(|_| gui::load_css());
    application.connect_activate(gui::build_ui);
    application.run()
}
