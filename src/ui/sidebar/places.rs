use super::sidebar_button;
use crate::files::places;
use gtk::{Box, Button, prelude::BoxExt};

pub fn build_places() -> Box {
    let places = Box::new(gtk::Orientation::Vertical, 0);

    places.append(&home_button());
    places.append(&recent_button());
    append_places(&places);

    places
}

fn home_button() -> Button {
    let (btn, icon_label) = sidebar_button();
    icon_label.image().set_icon_name(Some("user-home-symbolic"));

    let username = glib::user_name();
    let home_label = username.to_str().unwrap_or_else(|| "Home");
    icon_label.label().set_text(home_label);

    btn
}

fn recent_button() -> Button {
    let (btn, icon_label) = sidebar_button();
    icon_label
        .image()
        .set_icon_name(Some("document-open-recent-symbolic"));
    icon_label.label().set_text("Recent");

    btn
}

fn place_button(file_info: &gio::FileInfo) -> Button {
    let (btn, icon_label) = sidebar_button();
    match file_info.symbolic_icon() {
        Some(gicon) => icon_label.image().set_from_gicon(&gicon),
        None => icon_label.image().set_icon_name(Some("folder-symbolic")),
    };
    icon_label
        .label()
        .set_text(file_info.display_name().as_str());

    btn
}

fn append_places(gbox: &Box) {
    for file in places() {
        match file {
            Some(file_info) => gbox.append(&place_button(&file_info)),
            None => (),
        }
    }
}
