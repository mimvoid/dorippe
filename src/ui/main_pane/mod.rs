mod imp;
use crate::files::FileBrowser;
use crate::ui::file_item::FileItem;

use glib::object::{Cast, CastNone};
use gtk::{Box, GridView, Label, prelude::*, subclass::prelude::*};

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
    pub fn new(browser: &FileBrowser) -> Self {
        let main_pane = Self::default();
        main_pane.set_orientation(gtk::Orientation::Vertical);
        main_pane
            .imp()
            .selection
            .set_model(Some(&browser.sorted_file_list));

        let list_widget = main_pane.file_listings();
        let list_scroller = gtk::ScrolledWindow::builder().child(&list_widget).build();

        main_pane.append(&list_scroller);
        main_pane.append(&main_pane.status_bar(&browser.directories, &browser.files));

        main_pane
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

        let grid = GridView::builder()
            .model(&self.imp().selection)
            .factory(&factory)
            .enable_rubberband(true)
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .vexpand(true)
            .build();

        grid.connect_activate(|grid, position| {
            if let Some(selection) = grid.model().and_downcast_ref::<gtk::MultiSelection>() {
                let item = selection
                    .model()
                    .and_then(|list| list.item(position))
                    .and_downcast::<gio::FileInfo>();

                if let Some(file_info) = item {
                    let _ = grid.activate_action(
                        "win.go-to-child",
                        Some(&glib::Variant::from(file_info.display_name().as_str())),
                    );
                }
            };
        });

        grid
    }

    fn status_bar(&self, directories: &gtk::FilterListModel, files: &gtk::FilterListModel) -> Box {
        let status = Box::new(gtk::Orientation::Horizontal, 2);

        let dir_count = Label::new(None);
        status.append(&dir_count);
        directories
            .bind_property("n-items", &dir_count, "label")
            .transform_to(|_, number: u32| Some(glib::GString::from(format!("{number} folders"))))
            .sync_create()
            .build();

        let file_count = Label::new(None);
        status.append(&file_count);
        files
            .bind_property("n-items", &file_count, "label")
            .transform_to(|_, number: u32| Some(glib::GString::from(format!("{number} files"))))
            .sync_create()
            .build();

        status
    }
}
