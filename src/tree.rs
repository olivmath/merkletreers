use crate::merkle_proof::merkle_proof;
use crate::merkle_proof_check::merkle_proof_check;
use crate::merkle_root::merkle_root;
use crate::node::Node;
use crate::{Leaf, Proof, Root};

/// # 🌳 Merkle Tree
/// - You can pass raw data
/// - They will hashed by `keccak-256`
pub struct MerkleTree {
    pub leaves: Vec<Leaf>,
    pub root: Root,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Leaf>) -> Self {
        MerkleTree {
            leaves: leaves.clone(),
            root: merkle_root(&leaves),
        }
    }
}

impl MerkleTree {
    pub fn make_proof(&self, leaf: Leaf) -> Vec<Node> {
        merkle_proof(&self.leaves, leaf)
    }
}

impl MerkleTree {
    pub fn check_proof(self, proof: Proof, leaf: Leaf) -> Leaf {
        merkle_proof_check(proof, leaf)
    }
}
