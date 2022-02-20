use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum GetComponentDataOfEntityError {
    EntityNotFound,
    ComponentNotFound,
    ComponentNotInEntity,
}

impl Display for GetComponentDataOfEntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetComponentDataOfEntityError::EntityNotFound => {
                write!(f, "Entity with that id was not found")
            }
            GetComponentDataOfEntityError::ComponentNotFound => {
                write!(f, "Component with that name was not found")
            }
            GetComponentDataOfEntityError::ComponentNotInEntity => {
                write!(f, "The entity does not have that component")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SetComponentDataError {
    EntityNotFound,
    ComponentNotFound,
}

impl Display for SetComponentDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EntityNotFound => {
                write!(f, "Entity with that id was not found")
            }
            Self::ComponentNotFound => {
                write!(f, "Component with that name is already registered")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum RegisterEntityError {
    AlreadyRegistered,
}

impl Display for RegisterEntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRegistered => {
                write!(f, "Entity with that id is already registered")
            }
        }
    }
}

pub enum AddComponentError {
    NameAlreadyAdded,
}

impl Display for AddComponentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameAlreadyAdded => {
                write!(f, "Another component with that name already exists")
            }
        }
    }
}
