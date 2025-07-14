mod places;
use super::IconLabel;
use gtk::{Box, Button, prelude::BoxExt};

pub fn build_sidebar() -> Box {
    let sidebar = Box::new(gtk::Orientation::Vertical, 0);
    sidebar.append(&places::build_places());
    sidebar
}

fn sidebar_button(icon_label: &IconLabel) -> Button {
    icon_label.set_spacing(4);
    Button::builder()
        .child(icon_label)
        .css_classes(["flat"])
        .build()
}
