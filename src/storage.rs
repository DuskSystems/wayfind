use alloc::{vec, vec::Vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    index: usize,
}

#[derive(Debug, Clone)]
pub struct Storage<T> {
    data: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> Storage<T> {
    pub const fn new() -> Self {
        Self {
            data: vec![],
            free: vec![],
        }
    }

    pub fn insert(&mut self, data: T) -> Key {
        if let Some(index) = self.free.pop() {
            self.data[index] = Some(data);
            Key { index }
        } else {
            let index = self.data.len();
            self.data.push(Some(data));
            Key { index }
        }
    }

    pub fn get(&self, key: Key) -> Option<&T> {
        self.data.get(key.index)?.as_ref()
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        self.data.get_mut(key.index)?.as_mut()
    }

    pub fn remove(&mut self, key: Key) -> Option<T> {
        let slot = self.data.get_mut(key.index)?;
        if let Some(data) = slot.take() {
            self.free.push(key.index);
            Some(data)
        } else {
            None
        }
    }
}
