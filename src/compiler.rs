use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use crate::bounds::Bounds;
use crate::builder::BuilderNode;
use crate::node::{DynamicSearch, Node, WildcardSearch};
use crate::reachable::Reachable;
use crate::router::Router;
use crate::state::{DynamicState, RootState, StaticState, WildcardState};
use crate::suffixes::Suffixes;

/// Compiles a builder tree into an optimized tree.
pub(crate) struct Compiler {
    needles: BTreeMap<Box<[u8]>, usize>,
}

impl Compiler {
    pub(crate) fn run<T>(builder: BuilderNode<RootState, T>) -> Router<T> {
        let mut compiler = Self {
            needles: BTreeMap::new(),
        };

        let root = compiler.compile(builder);
        Router::new(root)
    }

    fn compile<S, T>(&mut self, builder: BuilderNode<S, T>) -> Node<S, T> {
        let mut static_children: Vec<Node<StaticState, T>> = builder
            .static_children
            .into_iter()
            .map(|child| self.compile(child))
            .collect();

        let mut seen = BTreeSet::new();
        let mut prefix = Vec::new();

        let mut dynamic_children: Vec<Node<DynamicState, T>> = builder
            .dynamic_children
            .into_iter()
            .map(|child| self.compile(child))
            .collect();

        for child in &mut dynamic_children {
            child.suffixes = Suffixes::compute(child, &mut prefix, &mut seen);
        }

        let mut wildcard_children: Vec<Node<WildcardState, T>> = builder
            .wildcard_children
            .into_iter()
            .map(|child| self.compile(child))
            .collect();

        for child in &mut wildcard_children {
            child.suffixes = Suffixes::compute(child, &mut prefix, &mut seen);
        }

        static_children.sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        dynamic_children.sort_by(|a, b| {
            b.suffixes
                .longest()
                .cmp(&a.suffixes.longest())
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        wildcard_children.sort_by(|a, b| {
            b.suffixes
                .longest()
                .cmp(&a.suffixes.longest())
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        let dynamic_search = if dynamic_children.iter().all(Node::is_segment_only) {
            DynamicSearch::Segment
        } else {
            DynamicSearch::Inline
        };

        let wildcard_search = if wildcard_children.iter().all(Node::is_segment_only) {
            WildcardSearch::Segment
        } else {
            WildcardSearch::Inline
        };

        let mut node = Node {
            state: builder.state,
            data: builder.data,

            static_children: static_children.into_boxed_slice(),
            dynamic_children: dynamic_children.into_boxed_slice(),
            wildcard_children: wildcard_children.into_boxed_slice(),
            end_wildcard: builder.end_wildcard,

            bounds: Bounds::default(),
            reachable: Reachable::default(),
            suffixes: Suffixes::default(),

            dynamic_search,
            wildcard_search,
        };

        node.bounds = Bounds::compute(&node);
        node.reachable = Reachable::compute(&node, &mut self.needles);

        node
    }
}
