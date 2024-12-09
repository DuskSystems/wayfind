use crate::id::RouteId;
use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
    marker::PhantomData,
};

pub type RouteMap<V> = HashMap<RouteId, V, BuildHasherDefault<NoHashHasher<RouteId>>>;

pub struct NoHashHasher<T>(u64, PhantomData<T>);

impl<T> Hasher for NoHashHasher<T> {
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use of NoHashHasher")
    }

    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

impl<T> Default for NoHashHasher<T> {
    fn default() -> Self {
        Self(0, PhantomData)
    }
}
