use crate::merkletree::Leaf;

/// # 🍃 Leaf of Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub r: Option<Leaf>,
    pub l: Option<Leaf>,
}

impl Node {
    pub fn none() -> Node {
        Node { r: None, l: None }
    }

    pub fn left(l: &str) -> Node {
        Node {
            r: None,
            l: Some(l.to_string()),
        }
    }

    pub fn right(r: &str) -> Node {
        Node {
            r: Some(r.to_string()),
            l: None,
        }
    }
}
