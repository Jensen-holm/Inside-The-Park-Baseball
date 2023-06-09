mod gui;

use gtk4::{glib, Application};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};

fn main() -> glib::ExitCode {
    let application = Application::new(
        Some("com.github.Jensen-holm.Inside-The-Park-Bsbl"),
        Default::default(),
    );

    application.connect_startup(|_| gui::load_css());
    application.connect_activate(|app| gui::build_home(app));
    application.run()
}
