use gtk::{Image, Label, glib, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(string = "
    template $IconLabel : Box {
        orientation: horizontal;

        Image image {}
        Label label {}
    }
    ")]
    pub struct IconLabel {
        #[template_child]
        pub image: TemplateChild<Image>,
        #[template_child]
        pub label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IconLabel {
        const NAME: &'static str = "IconLabel";
        type Type = super::IconLabel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IconLabel {}
    impl WidgetImpl for IconLabel {}
    impl BoxImpl for IconLabel {}
}

glib::wrapper! {
    pub struct IconLabel(ObjectSubclass<imp::IconLabel>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Orientable, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for IconLabel {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl IconLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_icon_name(icon_name: &str, label: Option<&str>) -> Self {
        let icon_label = Self::new();
        icon_label.image().set_icon_name(Some(icon_name));

        match label {
            Some(text) => icon_label.label().set_text(text),
            None => (),
        };

        icon_label
    }

    pub fn image(&self) -> &Image {
        &self.imp().image
    }

    pub fn label(&self) -> &Label {
        &self.imp().label
    }
}
