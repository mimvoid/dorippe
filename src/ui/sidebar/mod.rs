mod places;
use super::IconLabel;
use gtk::{
    Box, Button,
    prelude::{BoxExt, OrientableExt},
};

pub fn build_sidebar() -> Box {
    let sidebar = Box::new(gtk::Orientation::Vertical, 0);
    sidebar.append(&places::build_places());
    sidebar
}

fn sidebar_button() -> (Button, IconLabel) {
    let icon_label = IconLabel::new();
    icon_label.set_orientation(gtk::Orientation::Horizontal);
    icon_label.set_spacing(4);

    (Button::builder().child(&icon_label).build(), icon_label)
}
