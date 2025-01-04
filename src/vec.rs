use std::ops::{Index, IndexMut};

use crate::{node::Node, state::NodeState};

/// A `Node` which caches its sort state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SortedNode<'r, T, S: NodeState> {
    vec: Vec<Node<'r, T, S>>,
    sorted: bool,
}

impl<'r, T, S: NodeState> SortedNode<'r, T, S> {
    #[must_use]
    pub const fn new(vec: Vec<Node<'r, T, S>>) -> Self {
        Self { vec, sorted: false }
    }

    pub fn push(&mut self, value: Node<'r, T, S>) {
        self.vec.push(value);
        self.sorted = false;
    }

    pub fn remove(&mut self, index: usize) -> Node<'r, T, S> {
        self.vec.remove(index)
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<'r, T, S>>
    where
        F: Fn(&Node<'r, T, S>) -> bool,
    {
        self.vec.iter_mut().find(|item| predicate(item))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node<'r, T, S>> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<'r, T, S>> {
        self.vec.iter_mut()
    }

    pub fn sort(&mut self) {
        if self.sorted {
            return;
        }

        self.vec.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.state.cmp(&b.state))
        });

        self.sorted = true;
    }
}

impl<T, S: NodeState> Default for SortedNode<'_, T, S> {
    fn default() -> Self {
        Self {
            vec: Vec::new(),
            sorted: false,
        }
    }
}

impl<'r, T, S: NodeState> Index<usize> for SortedNode<'r, T, S> {
    type Output = Node<'r, T, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T, S: NodeState> IndexMut<usize> for SortedNode<'_, T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}
