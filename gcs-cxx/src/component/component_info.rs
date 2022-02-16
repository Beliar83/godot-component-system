use cxx::{type_id, ExternType};
use gcs::component::component_info::ComponentInfo;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct CXXComponentInfo {
    hash: u64,
}

unsafe impl ExternType for CXXComponentInfo {
    type Id = type_id!("gcs::ffi::ComponentInfo");
    type Kind = cxx::kind::Trivial;
}

impl ComponentInfo for CXXComponentInfo {
    fn get_hash(&self) -> u64 {
        self.hash
    }

    fn create(hash: u64) -> Self
    where
        Self: Sized,
    {
        CXXComponentInfo { hash }
    }
}

pub(crate) fn create_component_info(hash: u64) -> Box<CXXComponentInfo> {
    Box::new(CXXComponentInfo::create(hash))
}
