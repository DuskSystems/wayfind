use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage<T> {
    data: BTreeMap<Option<usize>, T>,
}

impl<T> Storage<T> {
    #[must_use]
    pub fn get(&self, id: &Option<usize>) -> Option<&T> {
        self.data.get(id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: &Option<usize>) -> Option<&mut T> {
        self.data.get_mut(id)
    }

    pub fn insert(&mut self, id: Option<usize>, data: T) {
        self.data.insert(id, data);
    }

    pub fn remove(&mut self, id: &Option<usize>) -> Option<T> {
        self.data.remove(id)
    }

    #[must_use]
    pub fn contains(&self, id: &Option<usize>) -> bool {
        self.data.contains_key(id)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &Option<usize>> {
        self.data.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Option<usize>, &T)> {
        self.data.iter()
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
}

impl<T> From<(Option<usize>, T)> for Storage<T> {
    fn from(pair: (Option<usize>, T)) -> Self {
        let mut map = BTreeMap::new();
        map.insert(pair.0, pair.1);
        Self { data: map }
    }
}

impl<T> Index<&Option<usize>> for Storage<T> {
    type Output = T;

    fn index(&self, id: &Option<usize>) -> &Self::Output {
        self.data.get(id).expect("no entry found for key")
    }
}

impl<T> IndexMut<&Option<usize>> for Storage<T> {
    fn index_mut(&mut self, id: &Option<usize>) -> &mut Self::Output {
        self.data.get_mut(id).expect("no entry found for key")
    }
}
