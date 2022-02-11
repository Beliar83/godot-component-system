use cxx::{type_id, ExternType};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ComponentInfo {
    pub hash: u64,
}

unsafe impl ExternType for ComponentInfo {
    type Id = type_id!("gcs::ffi::ComponentInfo");
    type Kind = cxx::kind::Trivial;
}

pub(crate) fn create_component_info(hash: u64) -> Box<ComponentInfo> {
    Box::new(ComponentInfo { hash })
}
