use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait Seed {
    fn to_value(self) -> u64;
}

impl Seed for i64 {
    fn to_value(self) -> u64 {
        self as u64
    }
}

impl Seed for i32 {
    fn to_value(self) -> u64 {
        self as u64
    }
}

impl Seed for u64 {
    fn to_value(self) -> u64 {
        self
    }
}

impl Seed for u32 {
    fn to_value(self) -> u64 {
        self as u64
    }
}

impl Seed for String {
    fn to_value(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Seed for &String {
    fn to_value(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Seed for &str {
    fn to_value(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}