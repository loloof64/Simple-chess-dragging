mod imp;
mod image_manager;

use std::sync::Arc;
use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct Board(ObjectSubclass<imp::Board>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}   

impl Board {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn update_image_size(&mut self, size: u32) {
        let image_manager =  Arc::clone(&self.imp().image_manager);
        let mut image_manager = image_manager.lock().unwrap();
        image_manager.update_image_size(size);
    }
}