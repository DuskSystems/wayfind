use std::ops::{Index, IndexMut};

/// A `vec` which caches its sort state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SortedVec<T: Ord> {
    vec: Vec<T>,
    sorted: bool,
}

impl<T: Ord> SortedVec<T> {
    pub const fn new(vec: Vec<T>) -> Self {
        Self { vec, sorted: false }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.sorted = false;
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.vec.remove(index)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn find_mut<F>(&mut self, predicate: F) -> Option<&mut T>
    where
        F: Fn(&T) -> bool,
    {
        self.vec.iter_mut().find(|item| predicate(item))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.vec.iter_mut()
    }

    pub fn sort(&mut self) {
        if self.sorted {
            return;
        }

        self.vec.sort();
        self.sorted = true;
    }
}

impl<T: Ord> Default for SortedVec<T> {
    fn default() -> Self {
        Self {
            vec: vec![],
            sorted: false,
        }
    }
}

impl<T: Ord> Index<usize> for SortedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T: Ord> IndexMut<usize> for SortedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}
