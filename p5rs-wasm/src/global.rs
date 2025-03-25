use std::sync::Mutex;

pub struct Global<T> {
    inner: Mutex<T>,
}

impl<T: Clone> Global<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    pub fn get(&self) -> T {
        (*self.inner.lock().unwrap()).clone()
    }

    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = value;
    }
}
