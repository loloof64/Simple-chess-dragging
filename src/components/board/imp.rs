use gtk::cairo;
use gtk::gdk::prelude::GdkCairoContextExt;
use gtk::gdk_pixbuf;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use gtk::subclass::prelude::*;
use image::load_from_memory;
use resvg::tiny_skia::Pixmap;
use resvg::usvg::{Size, Transform};
use resvg::usvg::{Options, Tree};

const DEFAULT_PIECE_SIZE: u32 = 200;
const NAVAJO_WHITE: (f64, f64, f64) = (1.0, 0.96, 0.86);
const PERU: (f64, f64, f64) = (0.80, 0.52, 0.25);

const SVG_DATA: &[u8] = include_bytes!("Chess_ndt45.svg");

#[derive(Default)]
pub struct Board {}

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
            .set_draw_func(|_area, ctx, width, height| draw_content(ctx, width, height));
    }
}

impl WidgetImpl for Board {}

impl DrawingAreaImpl for Board {}

fn draw_content(ctx: &cairo::Context, width: i32, height: i32) {
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
    draw_piece(ctx, 0.0, 0.0, cell_size);
}

fn draw_single_cell(ctx: &cairo::Context, x: f64, y: f64, size: f64, color: (f64, f64, f64)) {
    ctx.set_source_rgb(color.0, color.1, color.2);
    ctx.rectangle(x, y, size, size);
    ctx.fill().expect("failed to fill cell");
}

fn draw_piece(ctx: &cairo::Context, x: f64, y: f64, size: f64) {
    // load and parse svg
    let mut options = Options::default();
    options.resources_dir = None; // no external loading
    let rtree = Tree::from_data(SVG_DATA, &options).expect("Invalid SVG");

    // compute dimensions
    let svg_size = rtree.size();
    let target_size = Size::from_wh(size as f32, size as f32).expect("failed to compute svg size");

    let scale_x = target_size.width() / svg_size.width();
    let scale_y = target_size.height() / svg_size.height();

    let pixmap_size = Size::from_wh(size as f32, size as f32).expect("failed to compute svg size");

    // rasterize svg into pixmap
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
        .expect("Failed to create pixmap");
    let transform = Transform::from_scale(scale_x, scale_y);
    
    resvg::render(
        &rtree,
        transform,
        &mut pixmap.as_mut(),
    );

    // convert to gtk image
    let png_data = pixmap.encode_png().expect("Failed to encode to PNG");

    let image = load_from_memory(&png_data).expect("Failed to load PNG");
    let rgba8 = image.to_rgba8();
    let (width, height) = rgba8.dimensions();

    // convert to gtk pixbuf
    let pixbuf = Pixbuf::from_mut_slice(
        rgba8.into_raw(),
        gdk_pixbuf::Colorspace::Rgb,
        true,
        8,
        width as i32,
        height as i32,
        width as i32 * 4,
    );

    // paint
    ctx.save().unwrap();
    ctx.translate(x, y);
    ctx.set_source_pixbuf(&pixbuf, 0.0, 0.0);
    ctx.paint().expect("Failed to paint");
    ctx.restore().unwrap();
}
