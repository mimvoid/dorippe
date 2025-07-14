use crate::{files::places, ui::IconLabel};
use gtk::{Box, Button, prelude::*};

pub fn build_places() -> Box {
    let places_box = Box::new(gtk::Orientation::Vertical, 0);

    for file in places() {
        match file {
            Some((file_info, path_buf)) => {
                places_box.append(&place_button(&file_info, path_buf.as_path()))
            }
            None => (),
        }
    }

    places_box
}

fn place_button(file_info: &gio::FileInfo, path: &std::path::Path) -> Button {
    let icon_label = IconLabel::new();
    icon_label.set_spacing(4);
    icon_label.set_tooltip_text(path.to_str());

    match file_info.symbolic_icon() {
        Some(gicon) => icon_label.image().set_from_gicon(&gicon),
        None => icon_label.image().set_icon_name(Some("folder-symbolic")),
    };
    icon_label
        .label()
        .set_text(file_info.display_name().as_str());

    Button::builder()
        .child(&icon_label)
        .css_classes(["flat"])
        .build()
}
