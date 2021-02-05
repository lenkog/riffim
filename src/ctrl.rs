/*
 * Copyright (c) 2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use fltk::app::*;
use std::rc::Rc;

use super::jpeg::load_jpeg;
use super::ui::ImageDisplay;

pub struct Ctrl {
    files: Vec<String>,
    file_idx: usize,
    display: ImageDisplay,
}

impl Ctrl {
    pub fn new(files: Vec<String>, file_idx: usize, display: ImageDisplay) -> Ctrl {
        let mut ctrl = Ctrl {
            files: files,
            file_idx: file_idx,
            display: display,
        };
        ctrl.display
            .show(Rc::new(load_jpeg(&ctrl.files[ctrl.file_idx]).unwrap()));
        ctrl
    }

    pub fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::KeyDown => match event_key() {
                Key::Right => {
                    self.file_idx = (self.file_idx + 1) % self.files.len();
                    self.display
                        .show(Rc::new(load_jpeg(&self.files[self.file_idx]).unwrap()));
                    true
                }
                Key::Left => {
                    self.file_idx = if self.file_idx == 0 {
                        self.files.len() - 1
                    } else {
                        self.file_idx - 1
                    };
                    self.display
                        .show(Rc::new(load_jpeg(&self.files[self.file_idx]).unwrap()));
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}
