use std::sync::{LazyLock, Mutex, MutexGuard};

pub struct Global<T> {
    inner: LazyLock<Mutex<T>, GlobalNewFn<T>>,
}

type GlobalNewFn<T> = impl FnOnce() -> Mutex<T>;
#[define_opaque(GlobalNewFn)]
const fn helper<T>(x: T) -> GlobalNewFn<T> {
    move || Mutex::new(x)
}

impl<T> Global<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: LazyLock::new(helper(value)),
        }
    }

    pub fn get(&self) -> MutexGuard<'_, T> {
        self.inner.lock().unwrap()
    }
}
