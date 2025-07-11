use gtk::{Image, Label, glib, prelude::BoxExt, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Default, Debug)]
    pub struct IconLabel {
        pub icon: Image,
        pub label: Label,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IconLabel {
        const NAME: &'static str = "IconLabel";
        type Type = super::IconLabel;
        type ParentType = gtk::Box;
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
        let icon_label = Self::default();
        icon_label.append(icon_label.image());
        icon_label.append(icon_label.label());
        icon_label
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
        &self.imp().icon
    }

    pub fn label(&self) -> &Label {
        &self.imp().label
    }
}
