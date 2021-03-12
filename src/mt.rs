/*
 * Copyright (c) 2021 Lenko Grigorov.
 * This work is licensed under the 3-clause BSD License.
 * https://opensource.org/licenses/BSD-3-Clause
 */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub trait Shared<T> {
    fn to_shared(self) -> Arc<T>;
}

impl<T> Shared<T> for T {
    fn to_shared(self) -> Arc<T> {
        Arc::new(self)
    }
}

pub trait MutShared<T> {
    fn to_mut_shared(self) -> Arc<Mutex<T>>;
}

impl<T> MutShared<T> for T {
    fn to_mut_shared(self) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(self))
    }
}

#[derive(Clone)]
pub struct CancelMonitor {
    is_cancelled: Arc<AtomicBool>,
}

impl CancelMonitor {
    pub fn is_canceled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }
}

#[derive(Clone)]
pub struct CancelControl {
    is_cancelled: Arc<AtomicBool>,
}

impl CancelControl {
    pub fn new() -> CancelControl {
        CancelControl {
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_monitor(&self) -> CancelMonitor {
        CancelMonitor {
            is_cancelled: self.is_cancelled.clone(),
        }
    }

    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }
}

pub trait CancelableFn:
    FnOnce(CancelMonitor) + std::marker::Send + std::marker::Sync + 'static
{
}
impl<T: FnOnce(CancelMonitor) + std::marker::Send + std::marker::Sync + 'static> CancelableFn for T {}

pub fn start_cancelable(job: impl CancelableFn) -> CancelControl {
    let control = CancelControl::new();
    let monitor = control.get_monitor();
    std::thread::spawn(move || {
        job(monitor);
    });
    control
}
