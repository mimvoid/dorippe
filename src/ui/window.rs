use crate::files::FileBrowser;
use gio::prelude::FileExt;
use gtk::{glib, prelude::GtkWindowExt, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DorippeWindow {
        pub file_browser: FileBrowser,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DorippeWindow {
        const NAME: &'static str = "DorippeWindow";
        type Type = super::DorippeWindow;
        type ParentType = gtk::ApplicationWindow;
    }

    impl ObjectImpl for DorippeWindow {}
    impl WidgetImpl for DorippeWindow {}
    impl WindowImpl for DorippeWindow {}
    impl ApplicationWindowImpl for DorippeWindow {}
}

glib::wrapper! {
    pub struct DorippeWindow(ObjectSubclass<imp::DorippeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for DorippeWindow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl DorippeWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let win = Self::default();
        win.set_application(Some(app));
        win.set_title(Some("Dorippe"));
        win.imp().file_browser.list.set_attributes(Some("standard::*"));
        win
    }

    pub fn new_for_home(app: &gtk::Application) -> Self {
        let win = Self::new(app);
        win.imp().file_browser.set_to_home();
        win
    }

    pub fn file_browser(&self) -> &FileBrowser {
        return &self.imp().file_browser;
    }

    pub fn set_file(&mut self, file: &gio::File) {
        self.imp().file_browser.set_file(file);
    }

    pub fn go_to_parent(&mut self) {
        let file;
        match self.imp().file_browser.list.file() {
            Some(f) => file = f,
            None => return,
        };

        match file.parent() {
            Some(p) => self.set_file(&p),
            None => (),
        };
    }
}
