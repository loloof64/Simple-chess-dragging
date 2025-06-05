mod imp;
mod image_manager;

use std::sync::Arc;
use glib::Object;
use gtk::{glib::{self, subclass::types::ObjectSubclassIsExt}, DragSource, DropTarget};

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

    pub fn update_cell_size(&self, size: f64) {
        self.imp().cell_size.set(size);
    }

    pub fn get_cell_size(&self) -> f64 {
        self.imp().cell_size.get()
    }

    pub fn set_drag_source(&self, drag_source : DragSource) {
        *self.imp().drag_source.borrow_mut() = Some(drag_source);
    }

    pub fn set_drop_target(&self, drop_target : DropTarget) {
        *self.imp().drop_target.borrow_mut() = Some(drop_target);
    }

    pub fn get_value_at(&self, col: u8, row: u8) -> char {
        self.imp().cells_values.lock().unwrap()[row as usize][col as usize]
    }
}