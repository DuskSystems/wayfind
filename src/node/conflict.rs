use crate::{
    node::{Node, NodeData},
    parser::{Part, Template},
};

impl<S> Node<S> {
    pub fn conflict(&self, template: &mut Template) -> Option<&NodeData> {
        if let Some(part) = template.parts.pop() {
            match part {
                Part::Static { prefix } => self.conflict_static(template, &prefix),
                Part::Dynamic { .. } => self.conflict_dynamic(template),
                Part::Wildcard { .. } if template.parts.is_empty() => {
                    self.conflict_end_wildcard(template)
                }
                Part::Wildcard { .. } => self.conflict_wildcard(template),
            }
        } else {
            self.data.as_ref()
        }
    }

    fn conflict_static(&self, template: &Template, prefix: &[u8]) -> Option<&NodeData> {
        for child in &self.static_children {
            // This was previously a "starts_with" call, but turns out this is much faster.
            if prefix.len() >= child.state.prefix.len()
                && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            {
                let mut remaining_template = template.clone();

                if prefix.len() > child.state.prefix.len() {
                    let remaining = &prefix[child.state.prefix.len()..];
                    remaining_template.parts.push(Part::Static {
                        prefix: remaining.to_vec(),
                    });
                }

                if let Some(data) = child.conflict(&mut remaining_template) {
                    return Some(data);
                }
            }
        }

        None
    }

    fn conflict_dynamic(&self, template: &mut Template) -> Option<&NodeData> {
        if let Some(child) = self.dynamic_children.first() {
            return child.conflict(template);
        }

        None
    }

    fn conflict_wildcard(&self, template: &mut Template) -> Option<&NodeData> {
        if let Some(child) = self.wildcard_children.first() {
            return child.conflict(template);
        }

        None
    }

    fn conflict_end_wildcard(&self, template: &mut Template) -> Option<&NodeData> {
        if let Some(child) = self.end_wildcard_children.first() {
            return child.conflict(template);
        }

        None
    }
}
