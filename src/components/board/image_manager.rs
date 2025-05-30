use gtk::gdk_pixbuf::{self, Pixbuf};
use image::load_from_memory;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Size, Transform, Tree},
};

const START_SIZE: u32 = 45;
const SVG_DATA: &[u8] = include_bytes!("Chess_ndt45.svg");

#[derive(Clone)]
pub struct ImageManager {
    pub image: Pixbuf,
}

impl ImageManager {
    fn new() -> Self {
        let image = Self::generate_image(START_SIZE);
        Self { image }
    }

    fn generate_image(size: u32) -> Pixbuf {
        // load and parse svg
        let mut options = Options::default();
        options.resources_dir = None; // no external loading
        let rtree = Tree::from_data(SVG_DATA, &options).expect("Invalid SVG");

        // compute dimensions
        let svg_size = rtree.size();
        let target_size =
            Size::from_wh(size as f32, size as f32).expect("failed to compute svg size");

        let scale_x = target_size.width() / svg_size.width();
        let scale_y = target_size.height() / svg_size.height();

        let pixmap_size =
            Size::from_wh(size as f32, size as f32).expect("failed to compute svg size");

        // rasterize svg into pixmap
        let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
            .expect("Failed to create pixmap");
        let transform = Transform::from_scale(scale_x, scale_y);

        resvg::render(&rtree, transform, &mut pixmap.as_mut());

        // convert to gtk image
        let png_data = pixmap.encode_png().expect("Failed to encode to PNG");

        let image = load_from_memory(&png_data).expect("Failed to load PNG");
        let rgba8 = image.to_rgba8();
        let (width, height) = rgba8.dimensions();

        // convert to gtk pixbuf
        Pixbuf::from_mut_slice(
            rgba8.into_raw(),
            gdk_pixbuf::Colorspace::Rgb,
            true,
            8,
            width as i32,
            height as i32,
            width as i32 * 4,
        )
    }

    pub fn get_image_clone(&self) -> Pixbuf {
        self.image.clone()
    }
}

impl Default for ImageManager {
    fn default() -> Self {
        Self::new()
    }
}