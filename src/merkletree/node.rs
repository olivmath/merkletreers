use crate::merkletree::Leaf;

/// # 🍃 Leaf of Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub r: Option<Leaf>,
    pub l: Option<Leaf>,
}

