use std::ops::{Index, IndexMut};

use crate::{node::Node, state::NodeState};

/// A vec of `Node`'s, with cached sort state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nodes<S: NodeState> {
    vec: Vec<Node<S>>,
    sorted: bool,
}

impl<S: NodeState> Nodes<S> {
    #[must_use]
    pub const fn new(vec: Vec<Node<S>>) -> Self {
        Self { vec, sorted: false }
    }

    #[inline]
    pub fn push(&mut self, value: Node<S>) {
        self.vec.push(value);
        self.sorted = false;
    }

    pub fn remove(&mut self, index: usize) -> Node<S> {
        self.vec.remove(index)
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.vec.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Node<S>> {
        self.vec.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Node<S>> {
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

impl<S: NodeState> Default for Nodes<S> {
    fn default() -> Self {
        Self {
            vec: Vec::new(),
            sorted: false,
        }
    }
}

impl<'a, S: NodeState> IntoIterator for &'a Nodes<S> {
    type Item = &'a Node<S>;
    type IntoIter = std::slice::Iter<'a, Node<S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, S: NodeState> IntoIterator for &'a mut Nodes<S> {
    type Item = &'a mut Node<S>;
    type IntoIter = std::slice::IterMut<'a, Node<S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<S: NodeState> Index<usize> for Nodes<S> {
    type Output = Node<S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<S: NodeState> IndexMut<usize> for Nodes<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}
