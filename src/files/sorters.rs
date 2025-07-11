use gio::{FileInfo, FileType::Directory};
use glib::object::Cast;
use gtk::{CustomSorter, Ordering};

fn cmp_file_type(file1: &FileInfo, file2: &FileInfo) -> Ordering {
    match (file1.file_type(), file2.file_type()) {
        (Directory, Directory) => Ordering::Equal,
        (Directory, _) => Ordering::Smaller,
        (_, Directory) => Ordering::Larger,
        _ => Ordering::Equal,
    }
}

fn cmp_hidden(file1: &FileInfo, file2: &FileInfo) -> Ordering {
    match (file1.is_hidden(), file2.is_hidden()) {
        (true, true) | (false, false) => Ordering::Equal,
        (true, false) => Ordering::Smaller,
        (false, true) => Ordering::Larger
    }
}

pub fn file_sorter() -> CustomSorter {
    CustomSorter::new(move |obj1, obj2| {
        let file1 = obj1.downcast_ref::<FileInfo>().unwrap();
        let file2 = obj2.downcast_ref::<FileInfo>().unwrap();

        let file_type_order = cmp_file_type(file1, file2);
        if file_type_order != Ordering::Equal {
            return file_type_order;
        }

        let hidden_order = cmp_hidden(file1, file2);
        if hidden_order != Ordering::Equal {
            return hidden_order;
        }

        file1
            .display_name()
            .to_lowercase()
            .cmp(&file2.display_name().to_lowercase())
            .into()
    })
}

// pub fn file_section_sorter() -> CustomSorter {
//     CustomSorter::new(move |obj1, obj2| {
//         let file1 = obj1.downcast_ref::<FileInfo>().unwrap();
//         let file2 = obj2.downcast_ref::<FileInfo>().unwrap();
//
//         match (file1.file_type(), file2.file_type()) {
//             (Directory, Directory) => Ordering::Equal,
//             (Directory, _) => Ordering::Larger,
//             (_, Directory) => Ordering::Smaller,
//             _ => Ordering::Equal,
//         }
//     })
// }
