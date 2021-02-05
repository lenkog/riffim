/*
 * Copyright (c) 2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use fltk::{button::*, frame::*, image::RgbImage};
use std::rc::Rc;

use super::image::Image;

pub struct ImageDisplay {
    frame: Frame,
    // need to keep a ref to the displayed image because RgbImage doesn't
    image_ref: Rc<Image>,
}

impl ImageDisplay {
    pub fn new(width: i32, height: i32) -> ImageDisplay {
        ImageDisplay {
            frame: Frame::new(0, 0, width, height, ""),
            image_ref: Rc::new(Image::new()),
        }
    }

    pub fn show(&mut self, image: Rc<Image>) {
        self.image_ref = image.clone();
        let mut fltk_image = unsafe {
            RgbImage::from_data(
                &self.image_ref.data,
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
