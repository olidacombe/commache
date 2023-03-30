use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Decode a unique cache key from an instance
pub trait ToKey {
    fn key(&self) -> Vec<u8>;
}

impl ToKey for [&str] {
    fn key(&self) -> Vec<u8> {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_le_bytes().into()
    }
}
