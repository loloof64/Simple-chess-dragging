mod components;

use components::Board;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};


const APP_ID: &str = "com.loloof64.SimpleChessDragging";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let board = Board::new();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Simple chess dragging")
        .child(&board)
        .build();

    window.present();
}