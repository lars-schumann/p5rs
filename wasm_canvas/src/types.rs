use std::fmt::Display;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

const ORD: Ordering = Ordering::Relaxed;
pub struct Int {
    storage: AtomicI64,
}

impl Int {
    pub const fn new(value: i64) -> Self {
        Int {
            storage: AtomicI64::new(value),
        }
    }
    pub fn get(&self) -> i64 {
        self.storage.fetch_add(0, ORD)
    }

    pub fn set(&self, value: i64) {
        self.storage.store(value, ORD);
    }

    pub fn add(&self, value: i64) {
        self.storage.fetch_add(value, ORD);
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.storage.load(ORD))
    }
}

impl From<Int> for i64 {
    fn from(value: Int) -> Self {
        value.storage.load(ORD)
    }
}

impl Clone for Int {
    fn clone(&self) -> Self {
        Int {
            storage: AtomicI64::new(self.storage.load(ORD)),
        }
    }
}
pub struct Float {
    storage: AtomicU64,
}
impl Float {
    pub const fn new(value: f64) -> Self {
        Self {
            storage: AtomicU64::new(value.to_bits()),
        }
    }

    pub fn set(&self, value: f64) {
        self.storage.store(value.to_bits(), ORD);
    }

    pub fn get(&self) -> f64 {
        f64::from_bits(self.storage.fetch_add(0, ORD))
    }

    pub fn add(&self, value: f64) {
        self.storage
            .fetch_update(ORD, ORD, |v| Some((f64::from_bits(v) + value).to_bits()))
            .unwrap();
    }

    pub fn mul(&self, value: f64) {
        self.storage
            .fetch_update(ORD, ORD, |v| Some((f64::from_bits(v) * value).to_bits()))
            .unwrap();
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", f64::from_bits(self.storage.load(ORD)))
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        f64::from_bits(value.storage.load(ORD))
    }
}

impl Clone for Float {
    fn clone(&self) -> Self {
        Float {
            storage: AtomicU64::new(self.storage.load(ORD)),
        }
    }
}
