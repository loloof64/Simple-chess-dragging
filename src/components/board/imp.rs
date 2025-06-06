use gtk::gdk::{ContentProvider, MemoryTexture};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::{self, Type};
use gtk::prelude::{DrawingAreaExt, DrawingAreaExtManual, WidgetExt};
use gtk::subclass::prelude::*;
use gtk::{DragSource, DropTarget};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use super::drawing::{DEFAULT_PIECE_SIZE, draw_content};
use super::image_manager::ImageManager;

#[derive(Default)]
pub struct Board {
    pub image_manager: Arc<Mutex<ImageManager>>,
    pub cells_values: Arc<Mutex<[[char; 2]; 2]>>,
    pub drag_source: Rc<RefCell<Option<DragSource>>>,
    pub drop_target: Rc<RefCell<Option<DropTarget>>>,
    pub cell_size: Cell<f64>,
    pub start_pos: Rc<RefCell<Option<(u8, u8)>>>,
    pub end_pos: Rc<RefCell<Option<(u8, u8)>>>,
}

impl Board {
    fn set_default_size(&self) {
        self.obj()
            .set_size_request(DEFAULT_PIECE_SIZE as i32 * 2, DEFAULT_PIECE_SIZE as i32 * 2);
    }

    fn set_draw_function(&self) {
        let image_manager = Arc::clone(&self.image_manager);
        let cell_values = Arc::clone(&self.cells_values);

        let start_pos = Rc::clone(&self.start_pos);
        let end_pos = Rc::clone(&self.end_pos);
        self.obj().set_draw_func(move |_area, ctx, width, height| {
            let cell_values = cell_values.lock().unwrap();
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
            let start_pos = *start_pos.borrow();
            let end_pos = *end_pos.borrow();
            let piece_location = match start_pos {
                None => Some(piece_location),
                _ => None,
            };
            draw_content(
                ctx,
                width,
                height,
                Arc::clone(&image_manager),
                piece_location,
                start_pos,
                end_pos,
            );
        });
    }

    fn set_default_piece_location(&self) {
        let cell_values = Arc::clone(&self.cells_values);
        let mut cell_values = cell_values.lock().unwrap();
        cell_values[0][0] = 'n';
    }

    fn set_resize_function(&self) {
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
    }

    fn setup_drag_and_drop(&self) {
        let (drag_source, drop_target) = self.setup_drag_drop_objects();
        self.setup_drag_source_listener(drag_source);
        self.setup_drop_target_listener(drop_target);
    }

    fn setup_drag_drop_objects(&self) -> (DragSource, DropTarget) {
        let drag_source = DragSource::builder()
            .actions(gtk::gdk::DragAction::MOVE)
            .build();
        let drop_target = DropTarget::builder()
            .actions(gtk::gdk::DragAction::MOVE)
            .build();
        drop_target.set_types(&[Type::STRING]);

        self.obj().set_drag_source(drag_source.clone());
        self.obj().set_drop_target(drop_target.clone());

        self.obj().add_controller(drag_source.clone());
        self.obj().add_controller(drop_target.clone());

        (drag_source, drop_target)
    }

    fn setup_drag_source_listener(&self, drag_source: DragSource) {
        let board = Arc::new(Mutex::new(self.obj().clone()));
        let image_manager = Arc::clone(&self.image_manager);
        let drag_source_2 = Rc::clone(&self.drag_source);
        let start_pos = Rc::clone(&self.start_pos);
        drag_source.connect_prepare(move |_drag_source, x, y| {
            if let Ok(board) = board.lock() {
                let cell_size = board.get_cell_size();
                let half_cell_size = (cell_size / 2.0) as i32;
                let col = (x as f64 / cell_size) as u8;
                let row = (y as f64 / cell_size) as u8;
                let piece_value = board.get_value_at(row, col);
                if piece_value == 'n' {
                    // update start position
                    start_pos.replace(Some((row, col)));
                    // set transfered data
                    let text = piece_value.to_string();
                    let bytes = glib::Bytes::from(&text.as_ref());
                    let content_provider = ContentProvider::for_bytes("text/plain", &bytes);

                    // get drag and drop icon
                    let image_manager = image_manager.lock().unwrap();
                    let original_image = image_manager.get_image_clone();
                    let texture = Board::generate_drag_icon(original_image);

                    // finalize setup
                    if let Some(ref drag_source) = *drag_source_2.borrow_mut() {
                        drag_source.set_icon(Some(&texture), half_cell_size, half_cell_size);
                        Some(content_provider)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });
    }

    fn setup_drop_target_listener(&self, drop_target: DropTarget) {
        let board = Arc::new(Mutex::new(self.obj().clone()));
        let start_pos = Rc::clone(&self.start_pos);
        drop_target.connect_drop(move |_drop_target, value, x, y| {
            if let Ok(board) = board.lock() {
                let start_pos_2 = start_pos.borrow().unwrap();
                let cell_size = board.get_cell_size();
                let col = (x as f64 / cell_size) as u8;
                let row = (y as f64 / cell_size) as u8;
                let piece_value = value.get::<String>().unwrap().chars().next().unwrap();
                
                // if same location, cancel drag and drop
                let same_location = start_pos_2.0 == row && start_pos_2.1 == col;
                if same_location {
                    board.set_value_at(start_pos_2.0, start_pos_2.1, piece_value);
                    start_pos.replace(None);
                    return true;
                }
                // else validate drag and drop
                board.set_value_at(row, col, piece_value);
                board.set_value_at(start_pos_2.0, start_pos_2.1, 0 as char);

                start_pos.replace(None);
            }
            true
        });
        drop_target.connect_accept(move |_drop_target, _drop| true);
    }

    fn generate_drag_icon(original_image: Pixbuf) -> MemoryTexture {
        let format = if original_image.has_alpha() {
            gtk::gdk::MemoryFormat::R8g8b8a8
        } else {
            gtk::gdk::MemoryFormat::R8g8b8
        };

        let pixels = original_image.read_pixel_bytes();
        let bytes = glib::Bytes::from_owned(pixels);

        gtk::gdk::MemoryTexture::new(
            original_image.width(),
            original_image.height(),
            format,
            &bytes,
            original_image.rowstride() as usize,
        )
    }
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

        self.set_default_size();
        self.set_draw_function();
        self.set_default_piece_location();
        self.set_resize_function();
        self.setup_drag_and_drop();
    }
}

impl WidgetImpl for Board {}

impl DrawingAreaImpl for Board {}
