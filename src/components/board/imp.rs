use gtk::gdk::prelude::GdkCairoContextExt;
use gtk::glib;
use gtk::prelude::{DrawingAreaExt, DrawingAreaExtManual, WidgetExt};
use gtk::subclass::prelude::*;
use gtk::{DragSource, DropTarget, cairo};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use super::image_manager::ImageManager;

const DEFAULT_PIECE_SIZE: u32 = 200;
const NAVAJO_WHITE: (f64, f64, f64) = (1.0, 0.96, 0.86);
const PERU: (f64, f64, f64) = (0.80, 0.52, 0.25);

#[derive(Default)]
pub struct Board {
    pub image_manager: Arc<Mutex<ImageManager>>,
    pub cells_values: Arc<Mutex<[[char; 2]; 2]>>,
    pub drag_source: Rc<RefCell<Option<DragSource>>>,
    pub drop_target: Rc<RefCell<Option<DropTarget>>>,
    pub cell_size: Cell<f64>,
}

impl Board {}

#[glib::object_subclass]
impl ObjectSubclass for Board {
    const NAME: &'static str = "SimpleChessDraggingBoard";
    type Type = super::Board;
    type ParentType = gtk::DrawingArea;
}

impl ObjectImpl for Board {
    fn constructed(&self) {
        self.parent_constructed();

        // Default size
        self.obj()
            .set_size_request(DEFAULT_PIECE_SIZE as i32 * 2, DEFAULT_PIECE_SIZE as i32 * 2);

        let image_manager = Arc::clone(&self.image_manager);
        let cell_values = Arc::clone(&self.cells_values);
        let cell_values_2 = Arc::clone(&self.cells_values);

        // Draw board
        self.obj().set_draw_func(move |_area, ctx, width, height| {
            let cell_values = cell_values_2.lock().unwrap();
            let piece_location = if cell_values[0][0] == 'n' {
                (0, 0)
            } else if cell_values[0][1] == 'n' {
                (0, 1)
            } else if cell_values[1][0] == 'n' {
                (1, 0)
            } else if cell_values[1][1] == 'n' {
                (1, 1)
            } else {
                (u8::MAX, u8::MAX)
            };
            draw_content(
                ctx,
                width,
                height,
                Arc::clone(&image_manager),
                piece_location,
            );
        });

        // Default piece location
        let mut cell_values = cell_values.lock().unwrap();
        cell_values[0][0] = 'n';

        // Adapt size on resize
        let board = Arc::new(Mutex::new(self.obj().clone()));
        self.obj().connect_resize(move |_board, w, h| {
            let width = w as u32;
            let height = h as u32;
            let minimum_size = width.min(height);
            let cell_size = minimum_size as f64 / 2f64;
            if let Ok(mut board) = board.lock() {
                board.update_image_size(cell_size as u32);
                board.update_cell_size(cell_size);
            }
        });

        // Drag and drop
        let drag_source = DragSource::builder()
            .actions(gtk::gdk::DragAction::MOVE)
            .build();
        let drop_target = DropTarget::builder()
            .actions(gtk::gdk::DragAction::MOVE)
            .formats(&gtk::gdk::ContentFormats::new(&["text/plain"]))
            .build();
        self.obj().add_controller(drag_source.clone());
        self.obj().add_controller(drop_target.clone());

        self.obj().set_drag_source(drag_source.clone());
        self.obj().set_drop_target(drop_target.clone());

        let board_2 = Arc::new(Mutex::new(self.obj().clone()));
        let image_manager_2 = Arc::clone(&self.image_manager);
        let drag_source_2 = Rc::clone(&self.drag_source);
        drag_source.connect_prepare(move |_drag_source, x, y| {
            if let Ok(board) = board_2.lock() {
                let cell_size = board.get_cell_size();
                let col = (x as f64 / cell_size) as u8;
                let row = (y as f64 / cell_size) as u8;
                let piece_value = board.get_value_at(col, row);
                if piece_value == 'n' {
                    let value = glib::Value::from(&piece_value.to_string());
                    let content_provider = gtk::gdk::ContentProvider::for_value(&value);

                    let image_manager = image_manager_2.lock().unwrap();
                    let pixbuf = image_manager.get_image_clone();
                    let format = if pixbuf.has_alpha() {
                        gtk::gdk::MemoryFormat::R8g8b8a8
                    } else {
                        gtk::gdk::MemoryFormat::R8g8b8
                    };

                    let pixels = pixbuf.read_pixel_bytes();
                    let bytes = glib::Bytes::from_owned(pixels);

                    let texture = gtk::gdk::MemoryTexture::new(
                        pixbuf.width(),
                        pixbuf.height(),
                        format,
                        &bytes,
                        pixbuf.rowstride() as usize,
                    );
                    if let Some(ref drag_source) = *drag_source_2.borrow_mut() {
                        drag_source.set_icon(Some(&texture), 0, 0);
                    }

                    Some(content_provider)
                } else {
                    None
                }
            } else {
                None
            }
        });

        let board_3 = Arc::new(Mutex::new(self.obj().clone()));
        drop_target.connect_drop(move |_drop_target, value, x, y| {
            if let Ok(board) = board_3.lock() {
                let cell_size = board.get_cell_size();
                let col = (x as f64 / cell_size) as u8;
                let row = (y as f64 / cell_size) as u8;
                println!("Dropping at pointer ({x}, {y})");
                println!("Cell is ({col}, {row})");
                println!("Value is {value:?}");
            }
            true
        });
    }
}

impl WidgetImpl for Board {}

impl DrawingAreaImpl for Board {}

fn draw_content(
    ctx: &cairo::Context,
    width: i32,
    height: i32,
    image_manager: Arc<Mutex<ImageManager>>,
    piece_location: (u8, u8),
) {
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
    draw_piece(ctx, image_manager, piece_location, cell_size);
}

fn draw_single_cell(ctx: &cairo::Context, x: f64, y: f64, size: f64, color: (f64, f64, f64)) {
    ctx.set_source_rgb(color.0, color.1, color.2);
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
