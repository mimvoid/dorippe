use crate::files::FileBrowser;
use gio::prelude::FileExt;
use glib::object::IsA;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use std::path::Path;

    #[derive(Debug, Default)]
    pub struct DorippeWindow {
        pub file_browser: FileBrowser,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DorippeWindow {
        const NAME: &'static str = "DorippeWindow";
        type Type = super::DorippeWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.install_action(
                "win.go-to-path",
                Some(glib::VariantTy::STRING),
                move |win, _, param| {
                    if let Some(str) = param.and_then(|v| v.str()) {
                        win.set_file(&gio::File::for_path(&Path::new(str)));
                    };
                },
            );

            klass.install_action(
                "win.go-to-child",
                Some(glib::VariantTy::STRING),
                move |win, _, param| {
                    if let Some(name) = param.and_then(|v| v.str()) {
                        let parent = win
                            .file()
                            .unwrap_or_else(|| gio::File::for_path(glib::current_dir().as_path()));

                        let child = parent.child(&Path::new(name));
                        let file_type = child
                            .query_file_type(gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE);

                        match file_type {
                            gio::FileType::Directory => win.set_file(&child),
                            _ => (), // TODO: open non-directory files
                        };
                    };
                },
            );

            klass.install_action("win.go-to-parent", None, move |win, _, _| {
                win.go_to_parent();
            });

            klass.install_action("win.go-home", None, move |win, _, _| {
                win.go_home();
            });
        }

        fn new() -> Self {
            Self::default()
        }
    }

    impl ObjectImpl for DorippeWindow {}
    impl WidgetImpl for DorippeWindow {}
    impl WindowImpl for DorippeWindow {}
    impl ApplicationWindowImpl for DorippeWindow {}
}

glib::wrapper! {
    pub struct DorippeWindow(ObjectSubclass<imp::DorippeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DorippeWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder::<DorippeWindow>()
            .property("application", app)
            .property("title", "Dorippe")
            .build()
    }

    pub fn new_for_home<P: IsA<gtk::Application>>(app: &P) -> Self {
        let win = Self::new(app);
        win.imp().file_browser.set_to_home();
        win
    }

    pub fn file_browser(&self) -> &FileBrowser {
        return &self.imp().file_browser;
    }

    pub fn file(&self) -> Option<gio::File> {
        self.imp().file_browser.list.file()
    }

    pub fn set_file(&self, file: &gio::File) {
        self.imp().file_browser.set_file(file);
    }

    pub fn set_path(&self, path: &std::path::Path) {
        self.set_file(&gio::File::for_path(path));
    }

    pub fn go_to_parent(&self) {
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

    pub fn go_home(&self) {
        self.imp().file_browser.set_to_home();
    }
}
