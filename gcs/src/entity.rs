use std::hash::Hash;

pub trait EntityId: Default + PartialEq + Eq + Hash + Copy + Clone {
    fn create() -> Self
    where
        Self: Sized;
    fn as_string(&self) -> String;
    fn parse_str(input: &str) -> Result<Self, String>
    where
        Self: Sized;
}
