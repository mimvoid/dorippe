mod imp;
use gtk::{gio, glib, prelude::WidgetExt, subclass::prelude::*};

glib::wrapper! {
    pub struct FileItem(ObjectSubclass<imp::FileItem>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for FileItem {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl FileItem {
    pub fn set_file_info(&self, file_info: &gio::FileInfo) {
        let imp = self.imp();
        imp.name.set_text(file_info.display_name().as_str());

        imp.icon
            .set_opacity(if file_info.is_hidden() { 0.6 } else { 1.0 });

        match file_info.icon() {
            Some(n) => imp.icon.set_from_gicon(&n),
            None => imp.icon.set_icon_name(Some("text-x-preview")),
        };
    }
}
