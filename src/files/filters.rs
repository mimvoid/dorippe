use gio::FileInfo;
use glib::object::Cast;
use gtk::CustomFilter;

fn is_directory(obj: &glib::Object) -> bool {
    let file = obj.downcast_ref::<FileInfo>().unwrap();
    file.file_type() == gio::FileType::Directory
}

pub fn directory_filter() -> CustomFilter {
    CustomFilter::new(move |obj| is_directory(obj))
}

pub fn not_directory_filter() -> CustomFilter {
    CustomFilter::new(move |obj| !is_directory(obj))
}
