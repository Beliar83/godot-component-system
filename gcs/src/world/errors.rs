use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum GetComponentOfEntityError {
    EntityNotFound,
    ComponentNotFound,
    ComponentNotInEntity,
}

impl Display for GetComponentOfEntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetComponentOfEntityError::EntityNotFound => {
                write!(f, "Entity with that id was not found")
            }
            GetComponentOfEntityError::ComponentNotFound => {
                write!(f, "Component with that name was not found")
            }
            GetComponentOfEntityError::ComponentNotInEntity => {
                write!(f, "The entity does not have that component")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GetComponentDataError {
    ComponentNotFound,
}

impl Display for GetComponentDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetComponentDataError::ComponentNotFound => {
                write!(f, "Component with that name was not found")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SetComponentDataError {
    EntityNotFound,
    ComponentNotFound,
    DataInUse,
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
            Self::DataInUse => {
                write!(f, "The data is already exclusively borrowed")
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
