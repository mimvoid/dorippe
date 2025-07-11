use gio::File;
use glib::subclass::types::ObjectSubclass;
use gtk::{Box, DirectoryList, MultiSelection, glib, subclass::prelude::*};

#[derive(Debug)]
pub struct MainPane {
    pub file_list: DirectoryList,
    pub selection: MultiSelection,
}

impl MainPane {
    pub fn set_file(&self, file: &File) {
        self.file_list.set_file(Some(file));
        self.selection.set_model(Some(&self.file_list));
    }
}

impl Default for MainPane {
    fn default() -> Self {
        let file_list = DirectoryList::builder().build();
        let selection = MultiSelection::new(Some(file_list.clone()));

        Self {
            file_list,
            selection,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainPane {
    const NAME: &'static str = "MainPane";
    type Type = super::MainPane;
    type ParentType = Box;
}

impl ObjectImpl for MainPane {}
impl WidgetImpl for MainPane {}
impl BoxImpl for MainPane {}
