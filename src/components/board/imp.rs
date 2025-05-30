use gtk::cairo;
use gtk::gdk::prelude::GdkCairoContextExt;
use gtk::glib;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use gtk::subclass::prelude::*;

use std::sync::Arc;

use super::image_manager::ImageManager;

const DEFAULT_PIECE_SIZE: u32 = 200;
const NAVAJO_WHITE: (f64, f64, f64) = (1.0, 0.96, 0.86);
const PERU: (f64, f64, f64) = (0.80, 0.52, 0.25);

#[derive(Default)]
pub struct Board {
    image_manager: Arc<ImageManager>,
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
        let image_manager = Arc::clone(&self.image_manager);
        self.obj().set_draw_func(move |_area, ctx, width, height| {
            draw_content(ctx, width, height, Arc::clone(&image_manager))
        });
    }
}

impl WidgetImpl for Board {}

impl DrawingAreaImpl for Board {}

fn draw_content(ctx: &cairo::Context, width: i32, height: i32, image_manager: Arc<ImageManager>) {
    let minimum_size = width.min(height);
    let cell_size = minimum_size as f64 / 2f64;
    draw_single_cell(ctx, 0.0, 0.0, cell_size as f64, NAVAJO_WHITE);
    draw_single_cell(ctx, cell_size as f64, 0.0, cell_size as f64, PERU);
    draw_single_cell(ctx, 0.0, cell_size as f64, cell_size as f64, PERU);
    draw_single_cell(
        ctx,
        cell_size as f64,
        cell_size as f64,
        cell_size as f64,
        NAVAJO_WHITE,
    );
    draw_piece(ctx, 0.0, 0.0, image_manager);
}

fn draw_single_cell(ctx: &cairo::Context, x: f64, y: f64, size: f64, color: (f64, f64, f64)) {
    ctx.set_source_rgb(color.0, color.1, color.2);
    ctx.rectangle(x, y, size, size);
    ctx.fill().expect("failed to fill cell");
}

fn draw_piece(ctx: &cairo::Context, x: f64, y: f64, image_manager: Arc<ImageManager>) {
    let piece_pixbuf = image_manager.get_image_clone();

    ctx.save().unwrap();
    ctx.translate(x, y);
    ctx.set_source_pixbuf(&piece_pixbuf, 0.0, 0.0);
    ctx.paint().expect("failed to paint piece");
    ctx.restore().unwrap();
}
