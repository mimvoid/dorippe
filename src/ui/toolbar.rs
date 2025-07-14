mod imp {
    use gtk::subclass::prelude::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(file = "src/blp/toolbar.blp")]
    pub struct Toolbar {
        #[template_child]
        pub go_back: TemplateChild<gtk::Button>,
        #[template_child]
        pub go_next: TemplateChild<gtk::Button>,
        #[template_child]
        pub search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Toolbar {
        const NAME: &'static str = "Toolbar";
        type Type = super::Toolbar;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Toolbar {}
    impl WidgetImpl for Toolbar {}
    impl BoxImpl for Toolbar {}
}

glib::wrapper! {
    pub struct Toolbar(ObjectSubclass<imp::Toolbar>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for Toolbar {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Toolbar {
    pub fn new() -> Self {
        Self::default()
    }
}
