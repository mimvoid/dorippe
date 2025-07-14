use std::path::PathBuf;
use gio::{FileInfo, prelude::FileExt};
use glib::UserDirectory;

const PINNED_PLACES: [UserDirectory; 5] = [
    UserDirectory::Desktop,
    UserDirectory::Documents,
    UserDirectory::Downloads,
    UserDirectory::Music,
    UserDirectory::Pictures,
];

pub fn places() -> [Option<(FileInfo, PathBuf)>; 5] {
    PINNED_PLACES.map(|dir| {
        let path_buf = glib::user_special_dir(dir).unwrap();

        match gio::File::for_path(path_buf.as_path()).query_info(
            "standard::*",
            gio::FileQueryInfoFlags::NONE,
            gio::Cancellable::NONE,
        ) {
            Ok(info) => Some((info, path_buf)),
            Err(_) => None
        }
    })
}
