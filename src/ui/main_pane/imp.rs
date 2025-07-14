use glib::subclass::types::ObjectSubclass;
use gtk::{Box, MultiSelection, glib, subclass::prelude::*};

#[derive(Debug)]
pub struct MainPane {
    pub selection: MultiSelection,
}

impl Default for MainPane {
    fn default() -> Self {
        Self {
            selection: MultiSelection::new(None::<gio::ListModel>),
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
