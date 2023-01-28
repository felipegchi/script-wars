use std::marker::PhantomData;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Id<T> {
    id: usize,
    phantom: PhantomData<T>
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Entity;
