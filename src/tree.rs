use crate::hasher::{Hashable, Keccak256Hasher};
use crate::merkle_proof::merkle_proof;
use crate::merkle_proof_check::merkle_proof_check;
use crate::merkle_root::merkle_root;
use crate::node::Node;
use crate::{Leaf, Proof, Root};

/// # 🌳 Merkle Tree
/// - You can pass raw data
/// - They will be hashed by the provided hash function (default: keccak-256)
pub struct MerkleTree<H: Hashable = Keccak256Hasher> {
    pub leaves: Vec<Leaf>,
    pub root: Root,
    pub hasher: H,
}

impl MerkleTree<Keccak256Hasher> {
    /// Create a new Merkle Tree with the default Keccak256 hasher
    pub fn new(leaves: Vec<Leaf>) -> Self {
        Self::new_with_hasher(leaves, Keccak256Hasher)
    }
}

impl<H: Hashable> MerkleTree<H> {
    /// Create a new Merkle Tree with a custom hasher
    pub fn new_with_hasher(leaves: Vec<Leaf>, hasher: H) -> Self {
        let root = merkle_root(&leaves, &hasher);
        MerkleTree {
            leaves: leaves.clone(),
            root,
            hasher,
        }
    }

    pub fn make_proof(&self, leaf: Leaf) -> Vec<Node> {
        merkle_proof(&self.leaves, leaf, &self.hasher)
    }

    pub fn check_proof(&self, proof: Proof, leaf: Leaf) -> Leaf {
        merkle_proof_check(proof, leaf, &self.hasher)
    }
}
