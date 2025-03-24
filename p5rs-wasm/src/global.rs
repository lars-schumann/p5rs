use std::sync::Mutex;

pub struct Global<T: Copy> {
    inner: Mutex<T>,
}

impl<T: Copy> Global<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    pub fn get(&self) -> T {
        *self.inner.lock().unwrap()
    }

    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = value;
    }
}
