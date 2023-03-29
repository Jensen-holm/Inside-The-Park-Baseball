use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Entry, Grid, Label};


pub(crate) fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK");
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Baseball Sim Hub");
    window.set_default_size(
        1200,
        800,
    );

    let team_grid = Grid::new();
    team_grid.set_row_homogeneous(true);
    team_grid.set_column_homogeneous(true);

    // team entry label
    let team_entry_label = Label::new(Some("Enter Teams: "));
    team_grid.attach(&team_entry_label, 0, 0, 1, 1);

    // team entry boxes
    let team1 = Entry::new();
    team1.set_size_request(1, 10);
    team1.set_text("Team 1");
    team_grid.attach(&team1, 1, 0, 1, 1);

    window.add(&team_grid);
    window.show_all();
    gtk::main();
}

