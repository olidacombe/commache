pub mod rocks;

pub trait Cache {
    /// Fetch from the cache, or None if key is not yet used
    fn get(&self, k: &str) -> Option<String>;
    /// Make or replace an entry in the cache
    fn patch(&mut self, k: &str, v: &str);
}

// pub fn get() -> impl Cache {
//
// }
