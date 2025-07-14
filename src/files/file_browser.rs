use crate::files::filters;
use crate::files::sorters;

use gio::File;
use gtk::{DirectoryList, FilterListModel, SortListModel};
use std::path::Path;

#[derive(Debug)]
pub struct FileBrowser {
    pub list: DirectoryList,
    pub sorted_file_list: SortListModel,
    pub directories: FilterListModel,
    pub files: FilterListModel,
}

impl Default for FileBrowser {
    fn default() -> Self {
        let file_list = DirectoryList::new(None, None::<&File>);

        let sorted_file_list = SortListModel::builder()
            .model(&file_list)
            .sorter(&sorters::file_sorter())
            .build();

        let directories = FilterListModel::builder()
            .model(&sorted_file_list)
            .filter(&filters::directory_filter())
            .build();

        let files = FilterListModel::builder()
            .model(&sorted_file_list)
            .filter(&filters::not_directory_filter())
            .build();

        Self {
            list: file_list,
            sorted_file_list,
            directories,
            files,
        }
    }
}

impl FileBrowser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_home() -> Self {
        let home_path_buf = glib::home_dir();
        FileBrowser::from_path(home_path_buf.as_path())
    }

    pub fn from_path(path: &Path) -> Self {
        let file_view = Self::default();
        file_view.set_file(&File::for_path(path));
        file_view
    }

    pub fn set_to_home(&self) {
        self.list
            .set_file(Some(&File::for_path(glib::home_dir().as_path())));
    }

    pub fn set_file(&self, file: &File) {
        self.list.set_file(Some(file));
    }
}
