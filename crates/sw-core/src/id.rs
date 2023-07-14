//! Module for defining [Id<T>]s that are useful phantom types for identifiers.
use std::marker::PhantomData;

use self::private::EntityType;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Id<T: EntityType> {
    id: usize,
    phantom: PhantomData<T>,
}

/// Private module for the `EntityType` trait.
mod private {
    // This trait is used to make sure that the `Id` type can only be used with the `Entity` type.
    pub trait EntityType {}

    impl EntityType for super::Entity {}
}

/// Abstract data type for an entity.
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Entity {}
