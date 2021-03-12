/*
 * Copyright (c) 2020-2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

mod ctrl;
mod error;
mod files;
mod image;
mod jpeg;
mod mt;
mod ui;

use fltk::{app::*, button::*, window::*};

use ctrl::Ctrl;
use ui::ImageDisplay;

fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("Missing argument: path to image");

    let path = std::path::Path::new(&file);
    let folder = if path.is_dir() {
        path
    } else if path.is_file() {
        path.parent().unwrap()
    } else {
        panic!("Invalid path");
    };
    let files = files::images_in(folder);
    let start_file_idx = files
        .iter()
        .position(|x| x == path.to_str().unwrap())
        .unwrap_or(0);

    // init multithreading support in FLTK
    fltk::app::lock().unwrap();

    // screen size is adjusted in the fltk module - so we need to revert
    const SCREEN_SIZE_COEF: f64 = 0.96;
    let size = fltk::app::screen_size();
    let win_width = (size.0 * SCREEN_SIZE_COEF) as i32;
    let win_height = (size.1 * SCREEN_SIZE_COEF) as i32;

    let app = App::default();
    let mut win = Window::new(0, 0, win_width, win_height, "Riffim");
    let ui = ImageDisplay::new(win_width, win_height);
    win.end();
    win.fullscreen(true);
    win.show();

    let mut ctrl = Ctrl::new(files, start_file_idx, ui);
    win.handle(move |event| ctrl.handle(event));

    app.run().unwrap();
}
