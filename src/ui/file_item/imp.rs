use gtk::{glib, subclass::prelude::*};

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/blp/file_item.blp")]
pub struct FileItem {
    #[template_child]
    pub name: TemplateChild<gtk::Label>,
    #[template_child]
    pub icon: TemplateChild<gtk::Image>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileItem {
    const NAME: &'static str = "FileItem";
    type Type = super::FileItem;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for FileItem {}
impl WidgetImpl for FileItem {}
impl BoxImpl for FileItem {}
