use glib::subclass::types::ObjectSubclass;
use gtk::{Box, subclass::prelude::*};

#[derive(Default, Debug, gtk::CompositeTemplate)]
#[template(file = "src/blp/sidebar.blp")]
pub struct Sidebar {
    #[template_child]
    pub bookmarks: TemplateChild<gtk::Box>,
    #[template_child]
    pub home_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub home_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub disks: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for Sidebar {
    const NAME: &'static str = "Sidebar";
    type Type = super::Sidebar;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Sidebar {}
impl WidgetImpl for Sidebar {}
impl BoxImpl for Sidebar {}
