use std::ops::Index;

use cranelift_entity::{EntityRef, ListPool, PrimaryMap, packed_option::ReservedValue};

#[derive(Debug, Clone)]
pub struct EntityArena<E: EntityRef + ReservedValue, T> {
    pub map: PrimaryMap<E, T>,
    pub pool: ListPool<E>,
}

impl<E: EntityRef + ReservedValue, T> Default for EntityArena<E, T> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            pool: Default::default(),
        }
    }
}

impl<E: EntityRef + ReservedValue, T> EntityArena<E, T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E: EntityRef + ReservedValue, T> Index<E> for EntityArena<E, T> {
    type Output = T;

    fn index(&self, index: E) -> &Self::Output {
        &self.map[index]
    }
}
