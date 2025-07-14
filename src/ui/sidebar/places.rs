use crate::{files::places, ui::IconLabel};
use gtk::{Box, Button, prelude::*};

pub fn build_places() -> Box {
    let places_box = Box::new(gtk::Orientation::Vertical, 0);

    for file in places() {
        if let Some((file_info, path_buf)) = file {
            if let Some(path) = path_buf.as_path().to_str() {
                places_box.append(&place_button(&file_info, path));
            }
        }
    }

    places_box
}

fn place_button(file_info: &gio::FileInfo, path: &str) -> Button {
    let icon_label = IconLabel::new();
    icon_label.set_spacing(4);
    icon_label.set_tooltip_text(Some(path));

    match file_info.symbolic_icon() {
        Some(gicon) => icon_label.image().set_from_gicon(&gicon),
        None => icon_label.set_icon_name("folder-symbolic"),
    };
    icon_label.set_text(file_info.display_name().as_str());

    Button::builder()
        .child(&icon_label)
        .css_classes(["flat"])
        .action_name("win.go-to-path")
        .action_target(&glib::Variant::from(path))
        .build()
}
