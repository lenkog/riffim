/*
 * Copyright (c) 2020-2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pixel_components: u32,
    pub icc: Option<Vec<u8>>,
}

impl Image {
    pub fn new() -> Image {
        Image {
            data: vec![],
            width: 0,
            height: 0,
            pixel_components: 1,
            icc: None::<Vec<u8>>,
        }
    }
}
