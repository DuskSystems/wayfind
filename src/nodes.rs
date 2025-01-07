use std::ops::{Index, IndexMut};

use crate::{node::Node, state::NodeState};

/// A vec of `Node`'s, with cached sort state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nodes<T, S: NodeState> {
    vec: Vec<Node<T, S>>,
    sorted: bool,
}

impl<T, S: NodeState> Nodes<T, S> {
    #[must_use]
    pub const fn new(vec: Vec<Node<T, S>>) -> Self {
        Self { vec, sorted: false }
    }

    #[inline]
    pub fn push(&mut self, value: Node<T, S>) {
        self.vec.push(value);
        self.sorted = false;
    }

    pub fn remove(&mut self, index: usize) -> Node<T, S> {
        self.vec.remove(index)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Node<T, S>> {
        self.vec.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Node<T, S>> {
        self.sorted = false;
        self.vec.iter_mut()
    }

    #[inline]
    pub fn sort(&mut self) {
        if self.sorted {
            return;
        }

        self.vec.sort_by(|a, b| a.state.cmp(&b.state));
        self.sorted = true;
    }
}

impl<T, S: NodeState> Default for Nodes<T, S> {
    fn default() -> Self {
        Self {
            vec: Vec::new(),
            sorted: false,
        }
    }
}

impl<'a, T, S: NodeState> IntoIterator for &'a Nodes<T, S> {
    type Item = &'a Node<T, S>;
    type IntoIter = std::slice::Iter<'a, Node<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, S: NodeState> IntoIterator for &'a mut Nodes<T, S> {
    type Item = &'a mut Node<T, S>;
    type IntoIter = std::slice::IterMut<'a, Node<T, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, S: NodeState> Index<usize> for Nodes<T, S> {
    type Output = Node<T, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T, S: NodeState> IndexMut<usize> for Nodes<T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}
