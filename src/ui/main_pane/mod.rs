mod imp;
use crate::ui::file_item::FileItem;

use gio::File;
use glib::object::{Cast, CastNone};
use gtk::{Box, Button, GridView, Label, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct MainPane(ObjectSubclass<imp::MainPane>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Orientable, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for MainPane {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl MainPane {
    pub fn new() -> Self {
        let main_pane = Self::default();
        main_pane.set_orientation(gtk::Orientation::Vertical);

        main_pane
            .imp()
            .file_list
            .set_attributes(Some("standard::*"));

        let home_path_buf = glib::home_dir();
        let dir = File::for_path(home_path_buf.as_path());
        main_pane.set_file(&dir);

        main_pane.build_main_pane();

        main_pane
    }

    pub fn set_file(&self, file: &File) {
        self.imp().set_file(file);
    }

    pub fn go_to_parent(&self) {
        let file;
        match self.imp().file_list.file() {
            Some(f) => file = f,
            None => return,
        };

        match file.parent() {
            Some(p) => self.set_file(&p),
            None => (),
        };
    }

    fn build_main_pane(&self) {
        let list_widget = self.file_listings();
        let list_scroller = gtk::ScrolledWindow::builder().child(&list_widget).build();

        self.append(&self.header());
        self.append(&list_scroller);
        self.append(&self.status_bar());
    }

    fn header(&self) -> Box {
        let header = Box::new(gtk::Orientation::Horizontal, 2);

        let go_back = Button::with_label("<");
        header.append(&go_back);

        let go_forward = Button::with_label(">");
        header.append(&go_forward);

        // let to_parent = Button::with_label("^");
        // header.append(&to_parent);
        // to_parent .connect_clicked(|_go_back| {
        //     self.go_to_parent();
        // });

        header
    }

    fn file_listings(&self) -> GridView {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row = FileItem::default();
            item.set_child(Some(&row));
        });
        factory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let file_info = item.item().and_downcast::<gio::FileInfo>().unwrap();

            let child = item.child().and_downcast::<FileItem>().unwrap();
            child.set_file_info(&file_info);
        });

        GridView::builder()
            .model(&self.imp().selection)
            .factory(&factory)
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    fn status_bar(&self) -> Box {
        let status = Box::new(gtk::Orientation::Horizontal, 2);

        let count = Label::new(None);
        status.append(&count);
        self.imp()
            .file_list
            .bind_property("n-items", &count, "label")
            .transform_to(|_, number: u32| {
                let mut num_str = number.to_string();
                num_str.push_str(" files");
                Some(glib::GString::from(num_str))
            })
            .sync_create()
            .build();

        status
    }
}
