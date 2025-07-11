use gio::FileInfo;
use glib::object::Cast;
use gtk::CustomFilter;

pub fn directory_filter() -> CustomFilter {
    CustomFilter::new(move |obj| {
        let file = obj.downcast_ref::<FileInfo>().unwrap();
        file.file_type() == gio::FileType::Directory
    })
}

pub fn not_directory_filter() -> CustomFilter {
    CustomFilter::new(move |obj| {
        let file = obj.downcast_ref::<FileInfo>().unwrap();
        file.file_type() != gio::FileType::Directory
    })
}
