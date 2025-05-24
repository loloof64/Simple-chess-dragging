use gtk::DrawingArea;
use gtk::cairo;
use gtk::glib;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use gtk::subclass::prelude::*;

const DEFAULT_PIECE_SIZE: u32 = 200;

#[derive(Default)]
pub struct Board {
    piece_location: (u8, u8),
}

#[glib::object_subclass]
impl ObjectSubclass for Board {
    const NAME: &'static str = "SimpleChessDraggingBoard";
    type Type = super::Board;
    type ParentType = gtk::DrawingArea;
}

impl ObjectImpl for Board {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj()
            .set_size_request(DEFAULT_PIECE_SIZE as i32 * 2, DEFAULT_PIECE_SIZE as i32 * 2);
        self.obj()
            .set_draw_func(|_area, ctx, width, height| draw_background(ctx, width, height));
    }
}

impl WidgetImpl for Board {}

impl DrawingAreaImpl for Board {}

fn draw_background(ctx: &cairo::Context, width: i32, height: i32) {
    ctx.set_source_rgb(0.9, 0.2, 0.1);
    ctx.rectangle(0.0, 0.0, width as f64, height as f64);
    ctx.fill().expect("failed to fill background");
}
