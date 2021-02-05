/*
 * Copyright (c) 2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use std::fs;
use std::path::Path;

const IMAGE_EXT: [&str; 2] = [".jpg", ".jpeg"];

pub fn images_in(path: &Path) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    paths
        .filter_map(|x| {
            x.map(|x| x.path()).map_or(None, move |x| {
                {
                    if x.is_file() {
                        Some(x.to_str().unwrap().to_string())
                    } else {
                        None
                    }
                }
                .map_or(None, |x| {
                    if IMAGE_EXT.iter().any(|ext| x.to_lowercase().ends_with(ext)) {
                        Some(x)
                    } else {
                        None
                    }
                })
            })
        })
        .collect()
}
