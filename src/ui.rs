/*
 * Copyright (c) 2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use fltk::{button::*, frame::*, image::RgbImage};
use std::sync::{Arc, Mutex};

use super::image::{Image, SharedImage};
use super::mt::MutShared;

pub type SharedImageDisplay = Arc<Mutex<ImageDisplay>>;

pub struct ImageDisplay {
    frame: Frame,
    // need to keep a ref to the displayed image because RgbImage doesn't
    image_ref: SharedImage,
}

impl ImageDisplay {
    pub fn new(width: i32, height: i32) -> ImageDisplay {
        ImageDisplay {
            frame: Frame::new(0, 0, width, height, ""),
            image_ref: Image::new().to_mut_shared(),
        }
    }

    pub fn show(&mut self, shared_image: SharedImage) {
        self.image_ref = shared_image.clone();
        self.updated();
    }

    pub fn updated(&mut self) {
        let image = self.image_ref.lock().unwrap();
        let mut fltk_image = unsafe {
            RgbImage::from_data(
                &image.data,
                image.width,
                image.height,
                image.pixel_components,
            )
            .unwrap()
        };
        fltk_image.scale(self.frame.width(), self.frame.height(), true, false);
        self.frame.draw2(move |f| {
            fltk::draw::draw_rectf_with_rgb(0, 0, f.width(), f.height(), 128, 128, 128);
            fltk_image.draw(
                f.x() + (f.width() - fltk_image.width()) / 2,
                f.y() + (f.height() - fltk_image.height()) / 2,
                f.width(),
                f.height(),
            );
        });
        self.frame.redraw();
        self.frame.set_changed();
    }
}
