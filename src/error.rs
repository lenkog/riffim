/*
 * Copyright (c) 2020 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SimpleError {
    details: String,
}

impl SimpleError {
    pub fn to_box(msg: &str) -> Box<SimpleError> {
        Box::new(SimpleError {
            details: msg.to_string(),
        })
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        &self.details
    }
}
