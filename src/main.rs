mod gui;

use gtk4::{glib, Application};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual};

fn main() -> glib::ExitCode {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.builder_pattern"),
        Default::default(),
    );

    application.connect_activate(gui::build_ui);
    application.run()
}
