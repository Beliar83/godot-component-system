use std::hash::Hash;

pub trait ComponentInfo: Hash + Default + Eq + Copy + Clone {
    fn get_hash(&self) -> u64;
    fn create(hash: u64) -> Self
    where
        Self: Sized;
}
