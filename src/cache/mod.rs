pub mod rocks;

pub trait Cache<K>
where
    K: Send,
{
    /// Fetch from the cache, or None if key is not yet used
    fn get(&self, k: K) -> Option<String>;
    /// Make or replace an entry in the cache
    fn patch(&mut self, k: K, v: &[u8]);
}
