mod imp;
mod places;
use gio::subclass::prelude::*;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for Sidebar {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Sidebar {
    pub fn new() -> Self {
        let sidebar = Self::default();
        let imp = sidebar.imp();

        let username = glib::user_name();
        match username.to_str() {
            Some(str) => imp.home_label.set_text(str),
            None => (),
        }
        imp.home_button.set_tooltip_text(glib::home_dir().to_str());

        sidebar.append(&places::build_places());
        sidebar
    }
}
