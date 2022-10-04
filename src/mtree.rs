use crate::utils::is_power_2;

pub type Leaf = Vec<u8>;

/// # 🍃 Leaf of Tree
pub struct Node {
    pub right: Option<Leaf>,
    pub left: Vec<Leaf>,
}

/// # 🌳 Merkle Tree
/// - You can passa raw data
/// - They will hashed by `keccak-256`
pub struct MerkleTree {
    root: Option<Leaf>,
    leafs: Vec<Leaf>,
}

impl MerkleTree {
    pub fn new(leafs: Vec<String>) -> Self {
        is_power_2(leafs.len() as u32);
        MerkleTree {
            root: None,
            leafs: Self::hash_leaf(leafs),
        }
    }

    fn hash_leaf(leaf: Vec<String>) -> Vec<Leaf> {
        fn to_keccak256(message: Vec<u8>) -> Vec<u8> {
            use tiny_keccak::{Hasher, Keccak};

            let mut k256 = Keccak::v256();
            let mut result = [0; 32];

            k256.update(&message);
            k256.finalize(&mut result);

            result.to_vec()
        }
        leaf.into_iter().map(|leaf| to_keccak256(leaf.as_bytes().to_vec())).collect()
    }
}

impl MerkleTree {
    pub fn leafs(&self) -> Vec<String> {
        self.leafs.iter().map(|leaf| hex::encode(leaf)).collect()
    }

    pub fn root(&self) -> String {
        match &self.root {
            Some(root) => hex::encode(root),
            None => hex::encode(Vec::new()),
        }
    }
}
