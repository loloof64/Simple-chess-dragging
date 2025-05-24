use gtk::cairo;
use gtk::glib;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use gtk::subclass::prelude::*;

const DEFAULT_PIECE_SIZE: u32 = 200;
const NAVAJO_WHITE: (f64, f64, f64) = (1.0, 0.96, 0.86);
const PERU: (f64, f64, f64) = (0.80, 0.52, 0.25);

#[derive(Default)]
pub struct Board {
    
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
            .set_draw_func(|_area, ctx, width, height| draw_cells(ctx, width, height));
    }
}

impl WidgetImpl for Board {}

impl DrawingAreaImpl for Board {}

fn draw_cells(ctx: &cairo::Context, width: i32, height: i32) {
    let minimum_size = width.min(height);
    let cell_size = minimum_size as f64 / 2f64;
    draw_single_cell(ctx, 0.0, 0.0, cell_size as f64, NAVAJO_WHITE);
    draw_single_cell(ctx, cell_size as f64, 0.0, cell_size as f64, PERU);
    draw_single_cell(ctx, 0.0, cell_size as f64, cell_size as f64, PERU);
    draw_single_cell(ctx, cell_size as f64, cell_size as f64, cell_size as f64, NAVAJO_WHITE);
}

fn draw_single_cell(ctx: &cairo::Context, x: f64, y: f64, size: f64, color: (f64,f64,f64)) {
    ctx.set_source_rgb(color.0, color.1,  color.2);
    ctx.rectangle(x, y, size, size);
    ctx.fill().expect("failed to fill cell");
}
