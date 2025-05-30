mod imp;
mod image_manager;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Board(ObjectSubclass<imp::Board>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}   

impl Board {
    pub fn new() -> Self {
        Object::builder().build()
    }
}