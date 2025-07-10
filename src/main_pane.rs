// use gio::FileInfo;
use gtk::prelude::BoxExt;
use gtk::{Box, Button, Label};

pub fn build_main_pane() -> Box {
    let main_pane = Box::new(gtk::Orientation::Vertical, 0);

    main_pane.append(&header());
    main_pane.append(&file_listings());
    main_pane.append(&status_bar());

    main_pane
}

// fn file_button(file_info: &FileInfo) -> Button {
//     Button::builder()
//         .label(file_info.display_name())
//         .build()
// }

fn header() -> Box {
    let header = Box::new(gtk::Orientation::Horizontal, 2);

    let go_back = Button::with_label("<");
    let go_forward = Button::with_label(">");
    header.append(&go_back);
    header.append(&go_forward);

    header
}

fn file_listings() -> Box {
    let file_list = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(2)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .hexpand(true)
        .vexpand(true)
        .build();

    let button1 = Button::with_label("mock-file-1.txt");
    let button2 = Button::with_label("mock-file-2.txt");
    file_list.append(&button1);
    file_list.append(&button2);

    file_list
}

fn status_bar() -> Box {
    let status = Box::new(gtk::Orientation::Horizontal, 2);
    let mock_count = Label::new(Some("2 files"));
    status.append(&mock_count);

    status
}
