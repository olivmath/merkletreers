use merkletreers::hasher::{Hashable, Keccak256Hasher};
use merkletreers::tree::MerkleTree;
use merkletreers::utils::hash_it;

/// Example of a custom hasher using XOR (for demonstration purposes only)
/// WARNING: This is NOT cryptographically secure! Use only for testing.
#[derive(Clone, Copy, Debug)]
struct XorHasher;

impl Hashable for XorHasher {
    fn hash(&self, data: &[u8], buffer: &mut [u8; 32]) {
        // Simple XOR-based hash (NOT secure, just for demonstration)
        buffer.fill(0);
        for (i, byte) in data.iter().enumerate() {
            buffer[i % 32] ^= byte;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_with_default_hasher() {
        // Using default Keccak256 hasher
        let leaves = ["a", "b", "c", "d"]
            .iter()
            .map(|data| {
                let mut buffer = [0u8; 32];
                hash_it(data.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let tree = MerkleTree::new(leaves.clone());

        // Verify root was generated
        assert_ne!(tree.root, [0u8; 32]);

        // Verify we can make proof
        let proof = tree.make_proof(leaves[0]);
        assert!(!proof.is_empty());

        // Verify proof
        let computed_root = tree.check_proof(proof, leaves[0]);
        assert_eq!(computed_root, tree.root);
    }

    #[test]
    fn test_merkle_tree_with_custom_xor_hasher() {
        // Using custom XOR hasher
        let hasher = XorHasher;
        let leaves = ["a", "b", "c", "d"]
            .iter()
            .map(|data| {
                let mut buffer = [0u8; 32];
                hasher.hash(data.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let tree = MerkleTree::new_with_hasher(leaves.clone(), hasher);

        // Verify root was generated
        assert_ne!(tree.root, [0u8; 32]);

        // Verify we can make proof
        let proof = tree.make_proof(leaves[0]);
        assert!(!proof.is_empty());

        // Verify proof
        let computed_root = tree.check_proof(proof, leaves[0]);
        assert_eq!(computed_root, tree.root);
    }

    #[test]
    fn test_different_hashers_produce_different_roots() {
        // Same leaves with different hashers should produce different roots
        let keccak_hasher = Keccak256Hasher;
        let xor_hasher = XorHasher;

        // Create leaves with Keccak256
        let keccak_leaves = ["a", "b", "c", "d"]
            .iter()
            .map(|data| {
                let mut buffer = [0u8; 32];
                keccak_hasher.hash(data.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        // Create leaves with XOR
        let xor_leaves = ["a", "b", "c", "d"]
            .iter()
            .map(|data| {
                let mut buffer = [0u8; 32];
                xor_hasher.hash(data.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let keccak_tree = MerkleTree::new_with_hasher(keccak_leaves, keccak_hasher);
        let xor_tree = MerkleTree::new_with_hasher(xor_leaves, xor_hasher);

        // Different hashers should produce different roots
        assert_ne!(keccak_tree.root, xor_tree.root);
    }

    #[test]
    fn test_custom_hasher_proof_verification() {
        let hasher = XorHasher;
        let leaves = ["alice", "bob", "charlie", "dave"]
            .iter()
            .map(|data| {
                let mut buffer = [0u8; 32];
                hasher.hash(data.as_bytes(), &mut buffer);
                buffer
            })
            .collect::<Vec<[u8; 32]>>();

        let tree = MerkleTree::new_with_hasher(leaves.clone(), hasher);

        // Test proof for each leaf
        for leaf in &leaves {
            let proof = tree.make_proof(*leaf);
            let computed_root = tree.check_proof(proof, *leaf);
            assert_eq!(
                computed_root, tree.root,
                "Proof verification failed for a leaf"
            );
        }
    }
}
