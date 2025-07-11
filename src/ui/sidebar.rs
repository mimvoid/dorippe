use gtk::prelude::BoxExt;
use gtk::{Box, Button};

pub fn build_sidebar() -> Box {
    let sidebar = Box::new(gtk::Orientation::Vertical, 0);
    let recent = Button::with_label("Recent");
    let home = Button::with_label("Home");
    sidebar.append(&recent);
    sidebar.append(&home);

    sidebar
}
