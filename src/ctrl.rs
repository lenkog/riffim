/*
 * Copyright (c) 2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use fltk::app::*;

use super::jpeg::{apply_icc, load_jpeg};
use super::mt::*;
use super::ui::{ImageDisplay, SharedImageDisplay};

pub struct Ctrl {
    files: Vec<String>,
    file_idx: usize,
    display: SharedImageDisplay,
    icc_task: CancelControl,
}

impl Ctrl {
    pub fn new(files: Vec<String>, file_idx: usize, display: ImageDisplay) -> Ctrl {
        let mut ctrl = Ctrl {
            files: files,
            file_idx: file_idx,
            display: display.to_mut_shared(),
            icc_task: CancelControl::new(),
        };
        ctrl.display_file();
        ctrl
    }

    pub fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::KeyDown => match event_key() {
                Key::Right => {
                    self.file_idx = (self.file_idx + 1) % self.files.len();
                    self.display_file();
                    true
                }
                Key::Left => {
                    self.file_idx = if self.file_idx == 0 {
                        self.files.len() - 1
                    } else {
                        self.file_idx - 1
                    };
                    self.display_file();
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn display_file(&mut self) {
        self.icc_task.cancel();
        let file = self.files[self.file_idx].clone();
        let display = self.display.clone();
        self.icc_task = start_cancelable(move |cancel: CancelMonitor| {
            let load_result = load_jpeg(&file, cancel.clone());
            if cancel.is_canceled() {
                return;
            }
            let image = load_result.unwrap().to_mut_shared();
            let image_clone = image.clone();
            let display_clone = display.clone();
            fltk::app::awake(move || {
                display_clone.lock().unwrap().show(image_clone.clone());
            });
            apply_icc(&mut image.clone().lock().unwrap(), cancel.clone());
            if !cancel.is_canceled() {
                fltk::app::awake(move || {
                    display.lock().unwrap().updated();
                });
            }
        });
    }
}
