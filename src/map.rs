use crate::routers::{method::id::MethodId, path::id::PathId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChainMap<T>(HashMap<DataChain, T>);

impl<T> ChainMap<T> {
    pub fn get(&self, key: &DataChain) -> Option<&T> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: DataChain, value: T) -> Option<T> {
        self.0.insert(key, value)
    }

    pub fn contains_key(&self, key: &DataChain) -> bool {
        self.0.contains_key(key)
    }

    pub fn remove(&mut self, key: &DataChain) -> Option<T> {
        self.0.remove(key)
    }
}

impl<T> Default for ChainMap<T> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataChain {
    pub path: PathId,
    pub method: MethodId,
    str: String,
}

impl DataChain {
    pub fn new(path: PathId, method: MethodId) -> Self {
        Self {
            path,
            method,
            str: format!("{path}-{method}"),
        }
    }
}
