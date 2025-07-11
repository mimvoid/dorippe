use crate::files::FileView;

use gio::File;
use glib::subclass::types::ObjectSubclass;
use gtk::{Box, MultiSelection, glib, subclass::prelude::*};

#[derive(Debug)]
pub struct MainPane {
    pub file_view: FileView,
    pub selection: MultiSelection,
}

impl MainPane {
    pub fn set_file(&mut self, file: &File) {
        self.file_view.set_file(file);
        self.selection.set_model(Some(&self.file_view.sorted_file_list));
    }
}

impl Default for MainPane {
    fn default() -> Self {
        let file_view = FileView::new();
        let selection = MultiSelection::new(None::<gio::ListModel>);
        selection.set_model(Some(&file_view.sorted_file_list));

        Self {
            file_view,
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
