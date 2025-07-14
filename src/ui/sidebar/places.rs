use crate::{
    files::places,
    ui::{IconLabel, sidebar::sidebar_button},
};
use gtk::{Box, Button, prelude::*};

pub fn build_places() -> Box {
    let places = Box::new(gtk::Orientation::Vertical, 0);

    places.append(&home_button());
    places.append(&recent_button());
    append_places(&places);

    places
}

fn home_button() -> Button {
    let username = glib::user_name();
    let label = username.to_str().unwrap_or_else(|| "Home");

    let icon_label = IconLabel::from_icon_name("user-home-symbolic", Some(&label));
    icon_label.set_tooltip_text(glib::home_dir().to_str());

    sidebar_button(&icon_label)
}

fn recent_button() -> Button {
    let icon_label = IconLabel::from_icon_name("document-open-recent-symbolic", Some("Recent"));
    icon_label.set_tooltip_text(Some("Browse recent files"));
    sidebar_button(&icon_label)
}

fn place_button(file_info: &gio::FileInfo, path: &std::path::Path) -> Button {
    let icon_label = IconLabel::new();
    icon_label.set_tooltip_text(path.to_str());

    match file_info.symbolic_icon() {
        Some(gicon) => icon_label.image().set_from_gicon(&gicon),
        None => icon_label.image().set_icon_name(Some("folder-symbolic")),
    };
    icon_label
        .label()
        .set_text(file_info.display_name().as_str());

    sidebar_button(&icon_label)
}

fn append_places(gbox: &Box) {
    for file in places() {
        match file {
            Some((file_info, path_buf)) => {
                gbox.append(&place_button(&file_info, path_buf.as_path()))
            }
            None => (),
        }
    }
}
