use std::sync::{Arc, Mutex};

use gtk::cairo;
use gtk::gdk::prelude::GdkCairoContextExt;

use super::image_manager::ImageManager;

pub const DEFAULT_PIECE_SIZE: u32 = 200;
const NAVAJO_WHITE: (f64, f64, f64, f64) = (1.0, 0.96, 0.86, 1.0);
const PERU: (f64, f64, f64, f64) = (0.80, 0.52, 0.25, 1.0);
const RED: (f64, f64, f64, f64) = (1.0, 0.0, 0.0, 0.45);
const GREEN: (f64, f64, f64, f64) = (0.0, 1.0, 0.0, 0.45);

pub fn draw_content(
    ctx: &cairo::Context,
    width: i32,
    height: i32,
    image_manager: Arc<Mutex<ImageManager>>,
    piece_location: Option<(u8, u8)>,
    dnd_start_location: Option<(u8, u8)>,
    dnd_end_location: Option<(u8, u8)>,
) {
    let minimum_size = width.min(height);
    let cell_size = minimum_size as f64 / 2f64;
    draw_cells(ctx, cell_size, dnd_start_location, dnd_end_location);
    if let Some(piece_location) = piece_location {
        draw_piece(ctx, image_manager, piece_location, cell_size);
    }
}

fn draw_cells(
    ctx: &cairo::Context,
    cell_size: f64,
    dnd_start_location: Option<(u8, u8)>,
    dnd_end_location: Option<(u8, u8)>,
) {
    for row in 0..2 {
        for col in 0..2 {
            let mut background_color = if (row + col) % 2 == 0 {
                NAVAJO_WHITE
            } else {
                PERU
            };
            if let Some(dnd_start_location) = dnd_start_location {
                if dnd_start_location == (row, col) {
                    background_color = RED;
                }
            }
            if let Some(dnd_end_location) = dnd_end_location {
                if dnd_end_location == (row, col) {
                    background_color = GREEN;
                }
            }
            draw_single_cell(
                ctx,
                col as f64 * cell_size,
                row as f64 * cell_size,
                cell_size,
                background_color,
            );
        }
    }
}

fn draw_single_cell(ctx: &cairo::Context, x: f64, y: f64, size: f64, color: (f64, f64, f64, f64)) {
    ctx.set_source_rgba(color.0, color.1, color.2, color.3);
    ctx.rectangle(x, y, size, size);
    ctx.fill().unwrap();
}

fn draw_piece(
    ctx: &cairo::Context,
    image_manager: Arc<Mutex<ImageManager>>,
    piece_location: (u8, u8),
    cell_size: f64,
) {
    let image_manager = image_manager.lock().unwrap();
    let piece_pixbuf = image_manager.get_image_clone();

    ctx.save().unwrap();
    ctx.translate(
        piece_location.1 as f64 * cell_size,
        piece_location.0 as f64 * cell_size,
    );
    ctx.set_source_pixbuf(&piece_pixbuf, 0.0, 0.0);
    ctx.paint().unwrap();
    ctx.restore().unwrap();
}
