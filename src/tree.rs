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

    /// Verify a Merkle proof without needing a tree instance
    ///
    /// # Arguments
    /// * `proof` - The Merkle proof to verify
    /// * `leaf` - The leaf to verify
    /// * `root` - The expected root hash
    ///
    /// # Returns
    /// `true` if the proof is valid, `false` otherwise
    ///
    /// # Example
    /// ```
    /// use merkletreers::tree::MerkleTree;
    /// use merkletreers::node::{Node, Side};
    ///
    /// let leaf = [0u8; 32];
    /// let root = [0u8; 32];
    /// let proof = vec![Node { data: [0u8; 32], side: Side::LEFT }];
    ///
    /// let is_valid = MerkleTree::verify_proof(proof, leaf, root);
    /// ```
    pub fn verify_proof(proof: Proof, leaf: Leaf, root: Root) -> bool {
        Self::verify_proof_with_hasher(proof, leaf, root, &Keccak256Hasher)
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

    /// Verify a Merkle proof with a custom hasher without needing a tree instance
    ///
    /// # Arguments
    /// * `proof` - The Merkle proof to verify
    /// * `leaf` - The leaf to verify
    /// * `root` - The expected root hash
    /// * `hasher` - The hasher to use for verification
    ///
    /// # Returns
    /// `true` if the proof is valid, `false` otherwise
    pub fn verify_proof_with_hasher(proof: Proof, leaf: Leaf, root: Root, hasher: &H) -> bool {
        let computed_root = merkle_proof_check(proof, leaf, hasher);
        computed_root == root
    }
}
