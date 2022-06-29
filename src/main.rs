extern crate core;

use std::thread;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "dev.wford.ryuuji";

pub mod user_data;
mod library;
mod anime;

fn main() {
    //let appSettings = user_data::load_settings();

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {});

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ryuuji")
        .child(&button)
        .build();

    // Present window
    window.present();
}
